#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::webview::{DownloadEvent, PermissionKind, PermissionResponse};
use tauri::{AppHandle, LogicalPosition, Manager, WebviewBuilder, WebviewUrl, WindowEvent};

const WHATSAPP_LABEL: &str = "whatsapp-embedded";
#[cfg(not(target_os = "linux"))]
const WHATSAPP_UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Safari/537.36 Edg/154.0.0.0";

fn ensure_whatsapp(app: &AppHandle) -> Result<(), String> {
    // En Linux se crea una ventana nativa vacía: solo se ejecuta el WebView
    // de WhatsApp, sin mantener el prototipo local detrás de él.
    #[cfg(target_os = "linux")]
    if app.get_window("main").is_none() {
        tauri::WindowBuilder::new(app, "main")
            .title("Vamo Lite")
            .inner_size(1120.0, 720.0)
            .min_inner_size(720.0, 520.0)
            .resizable(true)
            .build()
            .map_err(|error| error.to_string())?;
    }
    let main = app
        .get_window("main")
        .ok_or_else(|| "No se encontró la ventana principal".to_string())?;

    if let Some(webview) = main.get_webview(WHATSAPP_LABEL) {
        webview.show().map_err(|e| e.to_string())?;
        return Ok(());
    }

    let size = main.inner_size().map_err(|e| e.to_string())?;
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("whatsapp-web-profile");
    let builder = WebviewBuilder::new(
        WHATSAPP_LABEL,
        WebviewUrl::External(
            "https://web.whatsapp.com"
                .parse()
                .expect("valid WhatsApp URL"),
        ),
    );
    #[cfg(not(target_os = "linux"))]
    let builder = builder.user_agent(WHATSAPP_UA);
    // WebKitGTK aplica sus ajustes de compatibilidad con WhatsApp al usar
    // su agente de usuario nativo; no se le fuerza la identidad de Chromium.
    let builder = builder
    .data_directory(data_dir)
    .zoom_hotkeys_enabled(true)
    .auto_resize()
    // Deja que WhatsApp Web reciba los eventos HTML5 de arrastrar y soltar.
    // Tauri instala por defecto un manejador nativo que intercepta los archivos
    // antes de que lleguen al selector de adjuntos de WhatsApp.
    .disable_drag_drop_handler()
    .on_download(|webview, event| {
        match event {
            DownloadEvent::Requested { destination, .. } => {
                let directory = match webview.app_handle().path().download_dir() {
                    Ok(directory) => directory,
                    Err(_) => return false,
                };
                if std::fs::create_dir_all(&directory).is_err() {
                    let _ = webview.eval("alert('No se pudo acceder a la carpeta Descargas.');");
                    return false;
                }
                let filename = destination.file_name().unwrap_or_else(|| std::ffi::OsStr::new("documento"));
                let original = directory.join(filename);
                let stem = original.file_stem().unwrap_or_default();
                let extension = original.extension();
                // Windows reserva el nombre. WebKitGTK rechaza destinos que ya
                // existen: en Linux dejamos que el motor cree el archivo y
                // mantenga su protección contra sobrescrituras.
                for number in 0..10000 {
                    let candidate = if number == 0 {
                        original.clone()
                    } else {
                        let mut name = stem.to_os_string();
                        name.push(format!(" ({number})"));
                        if let Some(extension) = extension {
                            name.push(".");
                            name.push(extension);
                        }
                        directory.join(name)
                    };
                    #[cfg(target_os = "linux")]
                    {
                        if candidate.exists() { continue; }
                        *destination = candidate;
                        return true;
                    }
                    #[cfg(not(target_os = "linux"))]
                    match std::fs::OpenOptions::new().write(true).create_new(true).open(&candidate) {
                        Ok(_) => { *destination = candidate; return true; }
                        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                        Err(_) => break,
                    }
                }
                let _ = webview.eval("alert('No se pudo guardar el archivo en Descargas.');");
                return false;
            }
            DownloadEvent::Finished { path: Some(path), success: true, .. } => {
                // Mostrar el archivo, sin ejecutar el documento descargado.
                #[cfg(target_os = "windows")]
                {
                    use std::os::windows::process::CommandExt;
                    let mut selection = std::ffi::OsString::from("/select,\"");
                    selection.push(path.as_os_str());
                    selection.push("\"");
                    // Explorer espera /select,"ruta", también con espacios y tildes.
                    let _ = std::process::Command::new("explorer.exe").raw_arg(selection).spawn();
                }
                #[cfg(target_os = "linux")]
                if let Some(directory) = path.parent() {
                    // Abrir la carpeta sin ejecutar el documento ni usar un shell.
                    let _ = std::process::Command::new("xdg-open")
                        .arg(directory)
                        .spawn();
                }
            }
            DownloadEvent::Finished { success: false, .. } => {
                let _ = webview.eval("alert('La descarga no se completó. Comprueba la conexión y vuelve a intentarlo.');");
            }
            _ => {}
        }
        true
    })
    .initialization_script(r#"
      (() => {
        const openEmojiPanel = () => {
          const selectors = [
            '[data-testid="smiley"]',
            '[aria-label*="emoji" i]',
            '[title*="emoji" i]',
            'button[aria-label*="sticker" i]'
          ];
          for (const selector of selectors) {
            const button = document.querySelector(selector);
            if (button) { button.click(); return true; }
          }
          return false;
        };
        document.addEventListener('keydown', (event) => {
          if (event.ctrlKey && event.shiftKey && event.key.toLowerCase() === 'e') {
            event.preventDefault();
            event.stopPropagation();
            openEmojiPanel();
          }
        }, true);
      })();
    "#)
    .on_permission_request(|_, kind| match kind {
        PermissionKind::Microphone | PermissionKind::Camera | PermissionKind::Notifications => PermissionResponse::Allow,
        _ => PermissionResponse::Default,
    });

    main.add_child(builder, LogicalPosition::new(0.0, 0.0), size)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn open_whatsapp(app: AppHandle) -> Result<(), String> {
    ensure_whatsapp(&app)
}

#[tauri::command]
async fn close_whatsapp(app: AppHandle) -> Result<(), String> {
    if let Some(main) = app.get_window("main") {
        if let Some(webview) = main.get_webview(WHATSAPP_LABEL) {
            webview.close().map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .on_window_event(|window, event| {
            if window.label() == "main" {
                if let WindowEvent::Resized(size) = event {
                    if let Some(webview) = window.get_webview(WHATSAPP_LABEL) {
                        let _ = webview.set_size(*size);
                    }
                }
            }
        })
        .setup(|app| {
            // Crear el WebView durante el arranque evita que una maximización
            // inmediata lo deje con el tamaño inicial de la ventana.
            if let Err(error) = ensure_whatsapp(&app.handle()) {
                eprintln!("No se pudo abrir WhatsApp Web: {error}");
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open_whatsapp, close_whatsapp])
        .run(tauri::generate_context!())
        .expect("error while running Vamo Lite");
}
