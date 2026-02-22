#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
BUILD="$ROOT/build"
ISO_DIR="$BUILD/iso"
ISO="$BUILD/teletubbyos.iso"

mkdir -p "$ISO_DIR"
rm -rf "$ISO_DIR"/*
mkdir -p "$ISO_DIR/boot"

# Build kernel (freestanding: core only)
( cd "$ROOT" && cargo build --release -p teletubby-kernel --manifest-path ./Cargo.toml )

cp "$ROOT/kernel/target/x86_64-teletubbyos/release/teletubby-kernel" "$ISO_DIR/boot/kernel.elf"
cp "$ROOT/boot/limine.cfg" "$ISO_DIR/boot/limine.cfg"

# Fetch/Build limine if missing
LIMINE_DIR="$ROOT/tools/limine"
if [[ ! -d "$LIMINE_DIR" ]]; then
  mkdir -p "$ROOT/tools"
  git clone --depth=1 https://github.com/limine-bootloader/limine.git "$LIMINE_DIR"
  ( cd "$LIMINE_DIR" && make -j"$(nproc)" )
fi

# Copy limine boot files
cp "$LIMINE_DIR/limine-bios.sys" "$ISO_DIR/boot/"
cp "$LIMINE_DIR/limine-bios-cd.bin" "$ISO_DIR/boot/"
cp "$LIMINE_DIR/limine-uefi-cd.bin" "$ISO_DIR/boot/"

mkdir -p "$ISO_DIR/EFI/BOOT"
cp "$LIMINE_DIR/BOOTX64.EFI" "$ISO_DIR/EFI/BOOT/"
cp "$LIMINE_DIR/BOOTIA32.EFI" "$ISO_DIR/EFI/BOOT/" 2>/dev/null || true

# Create ISO (xorriso)
xorriso -as mkisofs \
  -b boot/limine-bios-cd.bin \
  -no-emul-boot -boot-load-size 4 -boot-info-table \
  --efi-boot boot/limine-uefi-cd.bin \
  -efi-boot-part --efi-boot-image --protective-msdos-label \
  "$ISO_DIR" -o "$ISO"

# Install limine stage (BIOS)
"$LIMINE_DIR/limine" bios-install "$ISO"

# Run QEMU (serial to terminal)
qemu-system-x86_64 \
  -M q35 \
  -m 256M \
  -cdrom "$ISO" \
  -serial stdio \
  -no-reboot -no-shutdown
