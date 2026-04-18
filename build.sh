ROOT_DIR="$HOME/dev/rust/code/duxa_os_v2"
BOOT_DIR="$ROOT_DIR/bootloader"
KERNEL_DIR="$ROOT_DIR/kernel"

BOOT_EFI=$ROOT_DIR/target/x86_64-unknown-uefi/debug/bootloader.efi
KERNEL_EFI=$ROOT_DIR/target/x86_64-unknown-none/debug/kernel

cp /usr/share/OVMF/OVMF_CODE.fd $ROOT_DIR
cp /usr/share/OVMF/OVMF_VARS.fd $ROOT_DIR

rm esp.img

if [ ! -f esp.img ]; then
  dd if=/dev/zero of=esp.img bs=1M count=64
  mkfs.vfat esp.img
  mmd -i esp.img ::/EFI
  mmd -i esp.img ::/EFI/BOOT
fi

mcopy -o -i esp.img "$BOOT_EFI"   ::/EFI/BOOT/BOOTX64.EFI
mcopy -o -i esp.img "$KERNEL_EFI" ::/EFI/BOOT/KERNEL.EFI

qemu-system-x86_64 \
  -enable-kvm \
  -drive if=pflash,format=raw,readonly=on,file=$ROOT_DIR/OVMF_CODE.fd \
  -drive if=pflash,format=raw,file=$ROOT_DIR/OVMF_VARS.fd \
  -drive if=none,id=esp,format=raw,file=esp.img \
  -device ide-hd,drive=esp \
  -nographic
