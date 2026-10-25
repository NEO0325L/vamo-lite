# Vamo Lite

Aplicación independiente para Windows 11, con una primera adaptación experimental a Linux Mint 22.x, que abre WhatsApp Web dentro de una ventana de Tauri. Windows utiliza WebView2 y Linux utiliza WebKitGTK. Su objetivo es ofrecer una envoltura sencilla para equipos con recursos limitados.

Proyecto independiente, sin afiliación con WhatsApp ni Meta. Utiliza el sitio oficial de WhatsApp Web; no implementa el protocolo ni sincroniza conversaciones por cuenta propia.

## Descargar para Windows

[**Descargar Vamo Lite 0.1.1 para Windows 11 x64 (.exe)**](https://github.com/NEO0325L/vamo-lite/releases/download/v0.1.1/Vamo.Lite_0.1.1_x64-setup.exe)

[Notas de la versión y requisitos](https://github.com/NEO0325L/vamo-lite/releases/tag/v0.1.1). Versión experimental para Windows 11 x64. Descarga y ejecuta únicamente el instalador .exe: incluye el instalador completo de Microsoft Edge WebView2 y las bibliotecas de ejecución de Visual C++. WebView2 se instala automáticamente si falta. No necesitas instalar Rust, Visual Studio ni WebView2 por separado. La instalación de los componentes incluidos no requiere descargarlos de Internet; usar WhatsApp sí requiere conexión. El instalador no está firmado digitalmente. Cierra Vamo Lite antes de actualizar.

## Linux Mint 22.x (experimental)

Se está preparando el paquete Debian de 64 bits de la versión 0.1.2. Consulta [LINUX-MINT.md](LINUX-MINT.md) para instalación, compilación y diferencias de plataforma. El gestor de paquetes de Mint resuelve las dependencias del .deb; no necesitas instalar herramientas de desarrollo para usarlo. Esta adaptación requiere pruebas con una cuenta real en Linux antes de considerarse estable.

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

El instalador se genera en `src-tauri/target/release/bundle/nsis/Vamo Lite_0.1.1_x64-setup.exe`. Cierra la aplicación antes de actualizarla. Los binarios generados no se incluyen en Git.

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

El perfil de WebView2 se guarda dentro del directorio de datos de la aplicación, en `whatsapp-web-profile`. Contiene datos sensibles: Los documentos se guardan en Descargas del usuario.

La implementación actual permite automáticamente micrófono, cámara y notificaciones en la vista incrustada. No incluye una pantalla propia de revocación por sitio. Windows puede bloquear estos accesos desde sus ajustes de privacidad.

## Estado y limitaciones

Versión experimental 0.1.1 para Windows 11. El uso diario de las versiones anteriores se ha comprobado manualmente; este paquete completo requiere validación adicional en un sistema limpio. No se ha realizado una auditoría de seguridad ni una comparación controlada de memoria o CPU con el cliente oficial. No se garantiza un consumo concreto: WhatsApp Web y WebView2 determinan buena parte de él.

- Las llamadas y otras funciones dependen de su disponibilidad en WhatsApp Web.
- Cambios en WhatsApp pueden afectar al atajo de emojis y a la compatibilidad. El agente de usuario está fijado en el código.
- Se usa la función `unstable` de Tauri para la vista hija; revisa compatibilidad al actualizar.
- El instalador no está firmado digitalmente.
- Una descarga interrumpida puede dejar un archivo parcial o vacío.

## Contribuir

Consulta [CONTRIBUTING.md](CONTRIBUTING.md). 

## Licencia y marcas

**Autor y responsable del proyecto: [NEO0325L](https://github.com/NEO0325L).** Copyright (c) 2026 NEO0325L.

Vamo Lite nació como un proyecto personal y se comparte bajo la [licencia MIT](LICENSE). Se permite usar, modificar y redistribuir el código, incluso comercialmente, conservando el aviso de autoría y el texto de la licencia en las copias o partes sustanciales del software. La licencia no exige atribución visible en todas las pantallas. Consulta las condiciones completas en LICENSE.

Vamo Lite es un proyecto independiente y no está afiliado, patrocinado ni aprobado por WhatsApp o Meta. Las marcas y los servicios pertenecen a sus respectivos titulares. Las dependencias y WebView2 conservan sus propios términos; la licencia MIT del proyecto no concede derechos sobre componentes o marcas de terceros. Consulta [CREDITS.md](CREDITS.md) y [THIRD_PARTY_NOTICES.txt](THIRD_PARTY_NOTICES.txt).




