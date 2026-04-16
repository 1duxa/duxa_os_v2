BOOT_DIR="$HOME/dev/rust/code/duxa_os_v2/bootloader"

cargo +nightly build -Z build-std=core,compiler_builtins \
  --manifest-path="$BOOT_DIR/Cargo.toml" \
  --target x86_64-unknown-uefi

