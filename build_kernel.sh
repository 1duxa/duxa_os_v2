KERNEL_DIR="$HOME/dev/rust/code/duxa_os_v2/kernel"

cargo +nightly build -Z build-std=core,compiler_builtins \
  --manifest-path="$KERNEL_DIR/Cargo.toml" \
  --target x86_64-unknown-none
