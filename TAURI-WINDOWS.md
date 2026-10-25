# Compilar y diagnosticar en Windows

Los requisitos están en [README.md](README.md). Ejecuta desde la raíz del repositorio:

```powershell
rustc --version
cargo --version
cd .\src-tauri
cargo tauri info
cargo check --locked
cargo tauri build
```

El instalador NSIS aparece en `target/release/bundle/nsis/`. No se requiere WiX/MSI. Para desarrollo utiliza `cargo tauri dev`.

Si todas las dependencias ya están descargadas, puedes evitar consultas de red:

```powershell
$env:CARGO_NET_OFFLINE = 'true'
cargo tauri build
Remove-Item Env:CARGO_NET_OFFLINE
```

Este modo falla si falta alguna dependencia.

## Problemas habituales

- Proyecto Tauri no reconocido: sitúate en `src-tauri` y usa `cargo tauri build` sin `--manifest-path`.
- Faltan herramientas de enlace: instala escritorio con C++, MSVC y Windows SDK desde Visual Studio Installer.
- No carga el sitio: revisa conexión, disponibilidad de WhatsApp Web y WebView2 Runtime.
- Micrófono bloqueado: revisa los permisos para aplicaciones de escritorio en Windows. Borrar el perfil requiere vincular de nuevo la cuenta; no es el primer paso de diagnóstico.
- Descarga ausente: revisa Descargas y los permisos de escritura. La aplicación muestra un aviso si recibe un evento de descarga fallida.
- Actualización bloqueada: cierra primero todas las ventanas de Vamo Lite.

## Prueba manual antes de publicar

1. Abre y maximiza inmediatamente; restaura y redimensiona varias veces.
2. Vincula una cuenta de prueba y verifica la sesión después de reiniciar.
3. Envía y recibe texto e imágenes en un chat de prueba.
4. Arrastra un documento y verifica la vista previa antes de enviarlo.
5. Descarga un documento dos veces: deben conservarse ambos y abrirse sus ubicaciones. Incluye nombres con espacios y tildes.
6. Comprueba zoom y Ctrl + Shift + E con un chat abierto.
7. Comprueba notificaciones y una nota de voz con los permisos correspondientes.

Estas pruebas requieren interacción con WhatsApp. Que el código compile no demuestra por sí solo el funcionamiento completo.

## Instalador completo (desde 0.1.1)

`bundle.windows.webviewInstallMode.type` se configura como `offlineInstaller`. Tauri incorpora el instalador original completo de Microsoft Edge WebView2 x64 dentro del instalador NSIS. Durante el empaquetado se necesita conexión para obtenerlo de Microsoft cuando no está disponible en caché; ese requisito corresponde al desarrollador, no a quien instala Vamo Lite.

La persona usuaria ejecuta un solo .exe. Si WebView2 ya está instalado, se reutiliza; si falta, el instalador incluido se ejecuta automáticamente. Windows debe permitir la instalación y disponer de espacio suficiente. WhatsApp Web necesita Internet para iniciar sesión y enviar o recibir mensajes.

El paquete instala LICENSE, CREDITS.md y THIRD_PARTY_NOTICES.txt junto a la aplicación. El tamaño del instalador aumenta al incluir WebView2; esto no implica por sí mismo un aumento de memoria al ejecutar Vamo Lite.

Antes de afirmar compatibilidad en equipos sin WebView2, verifica el instalador en una máquina virtual limpia de Windows 11 x64. No desinstales el runtime de un equipo de uso diario para realizar esta prueba, ya que otras aplicaciones pueden depender de él.
