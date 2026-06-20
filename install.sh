#!/usr/bin/env bash
set -euo pipefail

export HOME="${HOME:-/tmp}"

if ! command -v cargo >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y -t wasm32-unknown-unknown --profile minimal
fi

if [ -f "$HOME/.cargo/env" ]; then
  # shellcheck disable=SC1091
  source "$HOME/.cargo/env"
fi

rustup target add wasm32-unknown-unknown

if ! command -v trunk >/dev/null 2>&1; then
  cargo install trunk --version 0.21.14
fi

trunk build --release
