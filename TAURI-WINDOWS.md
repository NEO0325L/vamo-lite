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
