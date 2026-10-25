# Contribuir

Describe el problema o la mejora antes de proponer cambios grandes. Mantén las funciones de mensajería, adjuntos y notificaciones. Mide cualquier optimización antes de afirmar una reducción de consumo.

## Reportar un problema

Incluye versiones de Windows, Vamo Lite y WebView2, pasos para reproducir y resultados esperado y observado. Oculta nombres, números y mensajes en las capturas. Nunca adjuntes QR, cookies, claves, documentos personales ni el perfil de sesión.

## Validación

Desde `src-tauri`, ejecuta `cargo check --locked` y `cargo tauri build`. Completa las pruebas manuales pertinentes de [TAURI-WINDOWS.md](TAURI-WINDOWS.md) e indica cuáles realizaste.

No incluyas `target/`, instaladores ni datos personales en contribuciones. Conserva `Cargo.lock` para facilitar la reproducción de las dependencias de la aplicación.
