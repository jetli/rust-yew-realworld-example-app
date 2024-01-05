#!/usr/bin/env just --justfile

install-wasm-pack:
  cargo install wasm-pack

install-tauri:
  cargo install tauri-cli

env:
  cp .env.example .env

trunk-serve:
  cd crates/conduit-wasm && trunk serve

build:
  cd crates/conduit-wasm && trunk build

test:
  wasm-pack test --headless --chrome

tauri-dev:
  cargo tauri dev

tauri-build:
  cargo tauri build

create-yew-app:
  npx create-yew-app my-app

serve-app:
  cd my-app && trunk serve