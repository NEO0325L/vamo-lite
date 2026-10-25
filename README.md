# Vamo Lite

Aplicación independiente para Windows 11, con una primera adaptación experimental a Linux Mint 22.x, que abre WhatsApp Web dentro de una ventana de Tauri. Windows utiliza WebView2 y Linux utiliza WebKitGTK. Su objetivo es ofrecer una envoltura sencilla para equipos con recursos limitados.

Proyecto independiente, sin afiliación con WhatsApp ni Meta. Utiliza el sitio oficial de WhatsApp Web; no implementa el protocolo ni sincroniza conversaciones por cuenta propia.

## Descargar para Windows

[**Descargar Vamo Lite 0.1.1 para Windows 11 x64 (.exe)**](https://github.com/NEO0325L/vamo-lite/releases/download/v0.1.1/Vamo.Lite_0.1.1_x64-setup.exe)

[Notas de la versión y requisitos](https://github.com/NEO0325L/vamo-lite/releases/tag/v0.1.1). Versión experimental para Windows 11 x64. Descarga y ejecuta únicamente el instalador .exe: incluye el instalador completo de Microsoft Edge WebView2 y las bibliotecas de ejecución de Visual C++. WebView2 se instala automáticamente si falta. No necesitas instalar Rust, Visual Studio ni WebView2 por separado. La instalación de los componentes incluidos no requiere descargarlos de Internet; usar WhatsApp sí requiere conexión. El instalador no está firmado digitalmente. Cierra Vamo Lite antes de actualizar.

## Linux Mint 22.x (experimental)

[**Descargar Vamo Lite 0.1.3 para Linux Mint 22.x x64 (.deb)**](https://github.com/NEO0325L/vamo-lite/releases/download/v0.1.3/vamo-lite_0.1.3_amd64.deb) · [Notas y SHA-256](https://github.com/NEO0325L/vamo-lite/releases/tag/v0.1.3).

Destinado a Mint 22.x de 64 bits (amd64), cuya base es Ubuntu 24.04. Abre el archivo con el instalador de paquetes de Mint o ejecuta desde la carpeta de descarga:

```bash
sudo apt install ./vamo-lite_0.1.3_amd64.deb
```

Después busca **Vamo Lite** en el menú y vincula WhatsApp mediante el QR del teléfono. El gestor de paquetes instala WebKitGTK, GTK y los complementos multimedia de GStreamer que falten; necesita Internet para descargarlos. No necesitas instalar Rust, Node.js, Visual Studio ni Microsoft WebView2 para usarlo.

Linux utiliza una ventana nativa con una sola vista de WhatsApp y el agente de usuario de WebKitGTK. Las descargas abren su carpeta con el gestor de archivos configurado. La sesión de Windows no se transfiere automáticamente a Linux.

Consulta [LINUX-MINT.md](LINUX-MINT.md) para integridad, compilación, diagnóstico y pruebas. Todavía falta validar una cuenta real en Mint. No se ha comprobado Mint 21.x, LMDE, ARM ni sistemas de 32 bits.

## Funciones de la aplicación

- WhatsApp Web al iniciar, con sesión persistente.
- Ventana redimensionable y ajuste al maximizar.
- Adjuntos mediante arrastrar y soltar.
- Descargas en la carpeta Descargas. En Windows se abre el Explorador con el archivo seleccionado; en Linux se abre la carpeta con el gestor del sistema. Los nombres repetidos reciben un sufijo numérico.
- Zoom: Ctrl + más, Ctrl + menos y Ctrl + 0.
- Panel de emojis: Ctrl + Shift + E, sujeto a la estructura actual de WhatsApp Web.
- Micrófono, cámara y notificaciones, si el sistema operativo también permite estos accesos.

## Compilar para Windows

Requisitos: Windows 11 x64, Rust estable con MSVC, Visual Studio Build Tools con Desarrollo para el escritorio con C++, MSVC, Windows SDK y Microsoft Edge WebView2 Runtime. No se necesita Node.js ni npm.

Desde la raíz del repositorio, en PowerShell:

```powershell
cargo install tauri-cli --version 2.12.1 --locked
cd .\src-tauri
cargo tauri build
```

El instalador se genera en `src-tauri/target/release/bundle/nsis/Vamo Lite_<versión>_x64-setup.exe`. Cierra la aplicación antes de actualizarla. Los binarios generados no se incluyen en Git.

Para desarrollo, ejecuta `cargo tauri dev` desde `src-tauri`. Consulta [TAURI-WINDOWS.md](TAURI-WINDOWS.md) para diagnóstico y pruebas.

## Compilar para Linux Mint

Compila dentro de Mint 22.x o Ubuntu 24.04 amd64 siguiendo [las instrucciones de Linux](LINUX-MINT.md#compilar-en-mint-22x). No se genera el paquete Linux desde la compilación de Windows. La configuración `tauri.linux.conf.json` selecciona el formato `.deb`, el icono, el acceso del menú y sus dependencias.

GitHub Actions genera el inventario de licencias para Linux antes de compilar. El paquete incluye la licencia MIT del proyecto, los créditos y los avisos de terceros.

## Vincular una cuenta

Abre Vamo Lite y escanea el QR mediante Dispositivos vinculados en WhatsApp del teléfono. Desde esa misma sección puedes desvincular el equipo. También puedes cerrar la sesión en WhatsApp Web.

## Estructura

| Ruta | Contenido |
| --- | --- |
| `src-tauri/src/main.rs` | WebView, inicio, permisos, descargas, redimensionado y atajo de emojis |
| `src-tauri/tauri.conf.json` | Ventana, recursos web y empaquetado NSIS |
| `src-tauri/tauri.linux.conf.json` | Configuración de Linux, dependencias DEB y permiso de zoom |
| `src-tauri/vamo-lite.desktop` | Acceso del menú de Linux |
| `src-tauri/Cargo.lock` | Versiones resueltas de las dependencias Rust |
| `src-tauri/icons/` | Iconos |
| `.github/workflows/linux-mint.yml` | Compilación, instalación y prueba de arranque del DEB |
| `.github/workflows/verify-linux-release.yml` | Comprobación del instalador Linux ya publicado |
| `scripts/` | Generador de avisos y pruebas de Linux |
| `web/` | Interfaz local de respaldo con conversaciones ficticias |
| `index.html`, `app.js`, `styles.css` | Prototipo original para navegador |

Las conversaciones reales las proporciona WhatsApp Web. Los indicadores de conexión y las cifras de memoria del prototipo local son demostrativos, no mediciones de la aplicación.

## Privacidad y permisos

El perfil del motor web se guarda dentro del directorio de datos de la aplicación, en `whatsapp-web-profile`, y contiene los datos de la sesión vinculada. En Linux su ubicación habitual es `~/.local/share/com.vamo.lite/whatsapp-web-profile`. Los documentos se guardan en Descargas del usuario.

La implementación actual permite automáticamente micrófono, cámara y notificaciones en la vista incrustada. No incluye una pantalla propia de revocación por sitio. El sistema operativo puede bloquear estos accesos desde sus ajustes de privacidad.

## Estado y limitaciones

| Plataforma | Versión descargable | Validación |
| --- | --- | --- |
| Windows 11 x64 | 0.1.1, experimental | Uso manual de versiones anteriores; el instalador completo aún requiere prueba en un sistema limpio |
| Linux Mint 22.x amd64 | 0.1.3, experimental | Compilación, instalación y arranque automatizados en Ubuntu 24.04; falta una prueba real en Mint con una cuenta vinculada |

La [compilación de Linux 0.1.3](https://github.com/NEO0325L/vamo-lite/actions/runs/40172780344) comprobó el contenido del paquete, su instalación con APT, sus bibliotecas dinámicas y un arranque de 15 segundos con pantalla virtual. Las verificaciones del instalador publicado están en [GitHub Actions](https://github.com/NEO0325L/vamo-lite/actions/workflows/verify-linux-release.yml).

Estas pruebas no demuestran todavía el funcionamiento del QR, la persistencia de una cuenta real, el envío de mensajes, los adjuntos, las notas de voz o las notificaciones en Mint. La [guía de migración](LINUX-MINT.md#comprobar-al-migrar) detalla las comprobaciones pendientes.

No se ha realizado una auditoría de seguridad ni una comparación controlada de memoria o CPU con el cliente oficial. No se garantiza un consumo concreto: WhatsApp Web y el motor web de cada plataforma determinan buena parte de él.

- Las llamadas y otras funciones dependen de su disponibilidad en WhatsApp Web.
- Cambios en WhatsApp pueden afectar al atajo de emojis y a la compatibilidad. En Windows se fija un agente de usuario de Chromium; Linux conserva el de WebKitGTK.
- Se usa la función `unstable` de Tauri para la vista hija; revisa compatibilidad al actualizar.
- El instalador de Windows no está firmado digitalmente. Para Linux se publica la suma SHA-256 junto con el DEB.
- Una descarga interrumpida puede dejar un archivo parcial o vacío.

## Contribuir

Consulta [CONTRIBUTING.md](CONTRIBUTING.md). 

## Licencia y marcas

**Autor y responsable del proyecto: [NEO0325L](https://github.com/NEO0325L).** Copyright (c) 2026 NEO0325L.

Vamo Lite nació como un proyecto personal y se comparte bajo la [licencia MIT](LICENSE). Se permite usar, modificar y redistribuir el código, incluso comercialmente, conservando el aviso de autoría y el texto de la licencia en las copias o partes sustanciales del software. La licencia no exige atribución visible en todas las pantallas. Consulta las condiciones completas en LICENSE.

Vamo Lite es un proyecto independiente y no está afiliado, patrocinado ni aprobado por WhatsApp o Meta. Las marcas y los servicios pertenecen a sus respectivos titulares. Las dependencias y WebView2 conservan sus propios términos; la licencia MIT del proyecto no concede derechos sobre componentes o marcas de terceros. Consulta [CREDITS.md](CREDITS.md) y [THIRD_PARTY_NOTICES.txt](THIRD_PARTY_NOTICES.txt).




