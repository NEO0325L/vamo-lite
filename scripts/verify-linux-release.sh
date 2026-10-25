#!/usr/bin/env bash
# Run only in an isolated X11 test display, after installing the released DEB.
set -euo pipefail
mkdir -p dist
app_pid=''
wm_pid=''
cleanup() {
  if [ -n "$app_pid" ]; then kill "$app_pid" 2>/dev/null || true; fi
  if [ -n "$wm_pid" ]; then kill "$wm_pid" 2>/dev/null || true; fi
}
trap cleanup EXIT

openbox > dist/WINDOW-MANAGER.txt 2>&1 &
wm_pid=$!
# Wait for the window manager rather than assuming it starts immediately.
for attempt in $(seq 1 50); do
  if wmctrl -m > dist/WINDOW-MANAGER-INFO.txt 2>/dev/null; then break; fi
  sleep 0.2
done
wmctrl -m
/usr/bin/vamo-lite > dist/STARTUP-LOG.txt 2>&1 &
app_pid=$!
window=$(timeout 30s xdotool search --sync --onlyvisible --pid "$app_pid" --name '^Vamo Lite$' | head -n 1)
test -n "$window"
hex_window=$(printf '0x%x' "$window")

check_size() {
  local expected_width=$1 expected_height=$2 geometry width height
  for attempt in $(seq 1 50); do
    geometry=$(xdotool getwindowgeometry --shell "$window")
    width=$(printf '%s\n' "$geometry" | sed -n 's/^WIDTH=//p')
    height=$(printf '%s\n' "$geometry" | sed -n 's/^HEIGHT=//p')
    if [ "$width" -eq "$expected_width" ] && [ "$height" -eq "$expected_height" ]; then
      printf '%s\n' "$geometry" >> dist/WINDOW-GEOMETRY.txt
      return 0
    fi
    sleep 0.2
  done
  printf 'Unexpected geometry: %s\n' "$geometry" >&2
  return 1
}

xdotool windowsize --sync "$window" 900 650
check_size 900 650
# Allow WebKit to paint the public login screen; no account is linked in CI.
sleep 15
kill -0 "$app_pid"
scrot --overwrite dist/WINDOW-900x650.png
wmctrl -i -r "$hex_window" -b add,maximized_vert,maximized_horz
for attempt in $(seq 1 50); do
  geometry=$(xdotool getwindowgeometry --shell "$window")
  width=$(printf '%s\n' "$geometry" | sed -n 's/^WIDTH=//p')
  height=$(printf '%s\n' "$geometry" | sed -n 's/^HEIGHT=//p')
  if [ "$width" -ge 1800 ] && [ "$height" -ge 1000 ]; then break; fi
  sleep 0.2
done
test "$width" -ge 1800
test "$height" -ge 1000
printf '%s\n' "$geometry" >> dist/WINDOW-GEOMETRY.txt
sleep 2
scrot --overwrite dist/WINDOW-MAXIMIZED.png
wmctrl -i -r "$hex_window" -b remove,maximized_vert,maximized_horz
xdotool windowsize --sync "$window" 900 650
check_size 900 650
sleep 2
scrot --overwrite dist/WINDOW-RESTORED.png

# Ask the window manager to close the application, like its title-bar button.
wmctrl -i -c "$hex_window"
for attempt in $(seq 1 50); do
  if ! kill -0 "$app_pid" 2>/dev/null; then break; fi
  sleep 0.2
done
if kill -0 "$app_pid" 2>/dev/null; then
  echo 'Application did not exit after closing its window.' >&2
  exit 1
fi
wait "$app_pid"
app_pid=''
if grep -Eiq 'panicked|No se pudo abrir WhatsApp Web|error while running Vamo Lite' dist/STARTUP-LOG.txt; then
  cat dist/STARTUP-LOG.txt
  exit 1
fi
printf 'PASS: installed release opens a visible window, resizes, maximizes, restores and exits cleanly.\n' | tee dist/RESULT.txt
