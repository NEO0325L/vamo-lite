# Vamo Lite

Aplicación experimental para Windows 11 que abre WhatsApp Web dentro de una ventana de Tauri y Microsoft Edge WebView2. Su objetivo es ofrecer una envoltura sencilla para equipos con recursos limitados.

Proyecto independiente, sin afiliación con WhatsApp ni Meta. Utiliza el sitio oficial de WhatsApp Web; no implementa el protocolo ni sincroniza conversaciones por cuenta propia.

## Funciones

- WhatsApp Web al iniciar, con sesión persistente.
- Ventana redimensionable y ajuste al maximizar.
- Adjuntos mediante arrastrar y soltar.
- Descargas en Descargas de Windows y apertura del Explorador con el archivo seleccionado. Los nombres repetidos reciben un sufijo numérico.
- Zoom: Ctrl + más, Ctrl + menos y Ctrl + 0.
- Panel de emojis: Ctrl + Shift + E, sujeto a la estructura actual de WhatsApp Web.
- Micrófono, cámara y notificaciones, si Windows también permite estos accesos.

## Compilar

Requisitos: Windows 11 x64, Rust estable con MSVC, Visual Studio Build Tools con Desarrollo para el escritorio con C++, MSVC, Windows SDK y Microsoft Edge WebView2 Runtime. No se necesita Node.js ni npm.

Desde la raíz del repositorio, en PowerShell:

```powershell
cargo install tauri-cli --version 2.12.1 --locked
cd .\src-tauri
cargo tauri build
```

El instalador se genera en `src-tauri/target/release/bundle/nsis/Vamo Lite_0.1.0_x64-setup.exe`. Cierra la aplicación antes de actualizarla. Los binarios generados no se incluyen en Git.

Para desarrollo, ejecuta `cargo tauri dev` desde `src-tauri`. Consulta [TAURI-WINDOWS.md](TAURI-WINDOWS.md) para diagnóstico y pruebas.

## Vincular una cuenta

Abre Vamo Lite y escanea el QR mediante Dispositivos vinculados en WhatsApp del teléfono. Desde esa misma sección puedes desvincular el equipo. También puedes cerrar la sesión en WhatsApp Web.

## Estructura

| Ruta | Contenido |
| --- | --- |
| `src-tauri/src/main.rs` | WebView, inicio, permisos, descargas, redimensionado y atajo de emojis |
| `src-tauri/tauri.conf.json` | Ventana, recursos web y empaquetado NSIS |
| `src-tauri/Cargo.lock` | Versiones resueltas de las dependencias Rust |
| `src-tauri/icons/` | Iconos |
| `web/` | Interfaz local de respaldo con conversaciones ficticias |
| `index.html`, `app.js`, `styles.css` | Prototipo original para navegador |

Las conversaciones reales las proporciona WhatsApp Web. Los indicadores de conexión y las cifras de memoria del prototipo local son demostrativos, no mediciones de la aplicación.

## Privacidad y permisos

El perfil de WebView2 se guarda dentro del directorio de datos de la aplicación, en `whatsapp-web-profile`. Contiene datos sensibles: no lo subas a GitHub ni lo adjuntes a reportes. Los documentos se guardan en Descargas del usuario.

La implementación actual permite automáticamente micrófono, cámara y notificaciones en la vista incrustada. No incluye una pantalla propia de revocación por sitio. Windows puede bloquear estos accesos desde sus ajustes de privacidad.

## Estado y limitaciones

Versión experimental 0.1.0, probada manualmente en Windows 11. No se ha realizado una auditoría de seguridad ni una comparación controlada de memoria o CPU con el cliente oficial. No se garantiza un consumo concreto: WhatsApp Web y WebView2 determinan buena parte de él.

- Las llamadas y otras funciones dependen de su disponibilidad en WhatsApp Web.
- Cambios en WhatsApp pueden afectar al atajo de emojis y a la compatibilidad. El agente de usuario está fijado en el código.
- Se usa la función `unstable` de Tauri para la vista hija; revisa compatibilidad al actualizar.
- El instalador no está firmado digitalmente.
- Una descarga interrumpida puede dejar un archivo parcial o vacío.

## Contribuir

Consulta [CONTRIBUTING.md](CONTRIBUTING.md). No publiques mensajes privados, documentos, QR, cookies ni perfiles de sesión.

## Licencia y marcas

La licencia de distribución y reutilización está pendiente de elección por su autor; por ahora no se declara una licencia de código abierto. Las dependencias conservan sus respectivas licencias. WhatsApp y otras marcas pertenecen a sus titulares.
