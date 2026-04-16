#!/bin/bash
cd "$HOME/dev/rust/code/duxa_os_v2"

cargo +nightly build \
  -Z build-std=core,compiler_builtins \
  --target x86_64-unknown-none \
  --package kernel
