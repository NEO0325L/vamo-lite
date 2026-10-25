# Vamo Lite en Linux Mint

## Sistema objetivo

Primera versión experimental para Linux Mint 22.x de 64 bits (amd64), basada en Ubuntu 24.04. No se ha validado en Mint 21.x, LMDE, ARM ni sistemas de 32 bits. Windows continúa usando su instalador .exe independiente.

## Instalar

Descarga [vamo-lite_0.1.3_amd64.deb](https://github.com/NEO0325L/vamo-lite/releases/download/v0.1.3/vamo-lite_0.1.3_amd64.deb) de la [publicación de Linux](https://github.com/NEO0325L/vamo-lite/releases/tag/v0.1.3) en GitHub y ábrelo con el instalador de paquetes de Mint. El gestor del sistema puede descargar las bibliotecas necesarias automáticamente; se requiere conexión para ello. No necesitas Rust, Node.js ni Visual Studio para usar la aplicación.

También puedes instalar desde la carpeta donde descargaste el archivo:

```bash
sudo apt install ./vamo-lite_0.1.3_amd64.deb
```

Usa el nombre real del archivo descargado. Evita `dpkg -i` como primer método: por sí solo no resuelve dependencias ausentes. Después busca Vamo Lite en el menú de aplicaciones.

## Comprobar la descarga

Descarga también [SHA256SUMS.txt](https://github.com/NEO0325L/vamo-lite/releases/download/v0.1.3/SHA256SUMS.txt) en la misma carpeta. Antes de instalar, ejecuta:

```bash
sha256sum --check SHA256SUMS.txt
```

El resultado debe mostrar `vamo-lite_0.1.3_amd64.deb: OK`. El hash comprueba que los bytes coinciden con el paquete publicado; no sustituye una auditoría del programa.

## Diferencias respecto a Windows

- Se utiliza WebKitGTK del sistema, en lugar de Microsoft WebView2. El paquete declara sus dependencias; no incluye un motor completo dentro del .deb ni promete instalación sin conexión.
- Al terminar una descarga se abre la carpeta correspondiente mediante xdg-open, compatible con el gestor de archivos configurado en Mint.
- Desde 0.1.3 el archivo de destino no se crea antes de descargar: WebKitGTK rechaza por defecto destinos existentes. Se buscan nombres libres y el motor mantiene su protección frente a sobrescrituras.
- Se crea solo una vista web de WhatsApp dentro de la ventana nativa, sin cargar el prototipo de chats ficticios detrás.
- El perfil de sesión se almacena bajo el directorio de datos del usuario, normalmente ~/.local/share/com.vamo.lite/whatsapp-web-profile. Tendrás que vincular de nuevo la cuenta al cambiar de Windows a Linux.
- WebKitGTK conserva su agente de usuario y sus ajustes de compatibilidad con WhatsApp.
- El zoom solo recibe un permiso de Tauri para esa función, limitado a la vista de WhatsApp y su origen HTTPS.

## Compilar en Mint 22.x

```bash
sudo apt update
sudo apt install build-essential libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libssl-dev patchelf xdg-utils
```

Instala Rust estable desde su distribución oficial, Node.js y Tauri CLI 2.12.1. En la raíz del repositorio:

```bash
cargo install tauri-cli --version 2.12.1 --locked
node scripts/generate-notices.mjs x86_64-unknown-linux-gnu
cd src-tauri
cargo tauri build --bundles deb -- --locked
```

Tauri carga automáticamente tauri.linux.conf.json. El paquete se genera en src-tauri/target/release/bundle/deb/.

## Compilación automatizada

El flujo Linux Mint 22 - paquete DEB compila en un ejecutor Ubuntu 24.04, comprueba arquitectura y contenido del paquete, lo instala con APT, comprueba las bibliotecas dinámicas, genera SHA-256 y realiza una prueba de arranque de 15 segundos con pantalla virtual. Sus permisos de repositorio son de lectura; no utiliza claves personales ni publica versiones automáticamente.

La versión 0.1.3 pasó estas comprobaciones en [GitHub Actions, Ubuntu 24.04](https://github.com/NEO0325L/vamo-lite/actions/runs/40172780344). No se ha probado todavía en una instalación real de Mint.

Un segundo flujo, [Verificar instalador publicado de Linux](https://github.com/NEO0325L/vamo-lite/actions/workflows/verify-linux-release.yml), descarga el DEB público, compara un SHA-256 fijado en el código, valida el acceso del menú y los decodificadores Opus, H.264 y AAC. En una pantalla X11 virtual con Openbox comprueba una ventana visible, redimensionado, maximización, restauración y cierre; guarda registros y capturas como evidencias. Esto tampoco reproduce todos los controladores gráficos ni las características de Cinnamon en un equipo real.

La prueba de arranque no verifica QR, mensajes, notas de voz, notificaciones ni reproducción multimedia con una cuenta real. Antes de considerar estable esta versión, prueba esas funciones en Mint. Los códecs y las capacidades multimedia dependen de WebKitGTK y de las bibliotecas del sistema.

## Comprobar al migrar

1. Abre Vamo Lite desde el menú y vincula tu cuenta mediante el QR.
2. Reinicia y verifica que la sesión persiste.
3. Maximiza, restaura y cambia el tamaño de la ventana.
4. Comprueba zoom y Ctrl + Shift + E con un chat abierto.
5. Adjunta y descarga un documento; verifica que se abra su carpeta.
6. Comprueba una nota de voz, imágenes y notificaciones.

Si aparece una ventana negra, prueba una vez desde terminal con `WEBKIT_DISABLE_DMABUF_RENDERER=1 vamo-lite` e informa del resultado junto con tu versión de Mint. Esa variable no se fuerza en el uso normal de la aplicación.
