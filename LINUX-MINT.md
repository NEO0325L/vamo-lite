# Vamo Lite en Linux Mint

## Sistema objetivo

Primera versión experimental para Linux Mint 22.x de 64 bits (amd64), basada en Ubuntu 24.04. No se ha validado en Mint 21.x, LMDE, ARM ni sistemas de 32 bits. Windows continúa usando su instalador .exe independiente.

## Instalar

Descarga el paquete .deb de la publicación de Linux en GitHub y ábrelo con el instalador de paquetes de Mint. El gestor del sistema puede descargar las bibliotecas necesarias automáticamente; se requiere conexión para ello. No necesitas Rust, Node.js ni Visual Studio para usar la aplicación.

También puedes instalar desde la carpeta donde descargaste el archivo:

```bash
sudo apt install ./Vamo\ Lite_0.1.2_amd64.deb
```

Usa el nombre real del archivo descargado. Evita `dpkg -i` como primer método: por sí solo no resuelve dependencias ausentes. Después busca Vamo Lite en el menú de aplicaciones.

## Diferencias respecto a Windows

- Se utiliza WebKitGTK del sistema, en lugar de Microsoft WebView2. El paquete declara sus dependencias; no incluye un motor completo dentro del .deb ni promete instalación sin conexión.
- Al terminar una descarga se abre la carpeta correspondiente mediante xdg-open, compatible con el gestor de archivos configurado en Mint.
- Se crea solo una vista web de WhatsApp dentro de la ventana nativa, sin cargar el prototipo de chats ficticios detrás.
- El perfil de sesión se almacena bajo el directorio de datos del usuario, normalmente ~/.local/share/com.vamo.lite/whatsapp-web-profile. Tendrás que vincular de nuevo la cuenta al cambiar de Windows a Linux.
- WebKitGTK conserva su agente de usuario y sus ajustes de compatibilidad con WhatsApp.
- El zoom solo recibe un permiso de Tauri para esa función, limitado a la vista de WhatsApp y su origen HTTPS.

## Compilar en Mint 22.x

```bash
sudo apt update
sudo apt install build-essential libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libssl-dev patchelf xdg-utils
```

Instala Rust estable desde su distribución oficial y Tauri CLI 2.12.1. En la raíz del repositorio:

```bash
cargo install tauri-cli --version 2.12.1 --locked
cd src-tauri
cargo tauri build --bundles deb -- --locked
```

Tauri carga automáticamente tauri.linux.conf.json. El paquete se genera en src-tauri/target/release/bundle/deb/.

## Compilación automatizada

El flujo Linux Mint 22 - paquete DEB compila en un ejecutor Ubuntu 24.04, comprueba arquitectura y contenido del paquete, genera SHA-256 y realiza una prueba de arranque con pantalla virtual. Sus permisos de repositorio son de lectura; no utiliza claves personales ni publica versiones automáticamente.

La prueba de arranque no verifica QR, mensajes, notas de voz, notificaciones ni reproducción multimedia con una cuenta real. Antes de considerar estable esta versión, prueba esas funciones en Mint. Los códecs y las capacidades multimedia dependen de WebKitGTK y de las bibliotecas del sistema.

## Comprobar al migrar

1. Abre Vamo Lite desde el menú y vincula tu cuenta mediante el QR.
2. Reinicia y verifica que la sesión persiste.
3. Maximiza, restaura y cambia el tamaño de la ventana.
4. Comprueba zoom y Ctrl + Shift + E con un chat abierto.
5. Adjunta y descarga un documento; verifica que se abra su carpeta.
6. Comprueba una nota de voz, imágenes y notificaciones.

Si aparece una ventana negra, prueba una vez desde terminal con `WEBKIT_DISABLE_DMABUF_RENDERER=1 vamo-lite` e informa del resultado junto con tu versión de Mint. Esa variable no se fuerza en el uso normal de la aplicación.
