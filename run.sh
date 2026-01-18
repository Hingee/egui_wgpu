#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WINDOW_DIR="$SCRIPT_DIR/wgpu-example"
SERVER_DIR="$SCRIPT_DIR/wgpu-server"

build() {
  cd "$WINDOW_DIR" || {
    echo "Cannot enter web_gpu_window"
    exit 1
  }
  wasm-pack build --target web --out-dir ../wgpu-server/static/pkg/
  if [ $? -ne 0 ]; then
    echo "Web-pack build failed"
    exit 1
  fi
  cd - >/dev/null
}

runWindow() {
  cd "$WINDOW_DIR" || {
    echo "Cannot enter web_gpu_window"
    exit 1
  }
  cargo run --release
  if [ $? -ne 0 ]; then
    echo "Window run Failed"
    exit 1
  fi
  cd - >/dev/null
}

runWeb() {
  cd "$SERVER_DIR" || {
    echo "Cannot enter wgpu-server"
    exit 1
  }
  cargo run --release
  if [ $? -ne 0 ]; then
    echo "Web run Failed"
    exit 1
  fi
}

case "$1" in
-build)
  build
  ;;

-runWeb)
  runWeb
  ;;

-runWindow)
  runWindow
  ;;

*)
  echo "Usage: $0 {-build|-runWeb|-runWindow}"
  exit 1
  ;;
esac
