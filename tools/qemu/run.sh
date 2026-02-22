#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
BUILD="$ROOT/build"
ISO_DIR="$BUILD/iso"
ISO="$BUILD/teletubbyos.iso"

mkdir -p "$ISO_DIR"
rm -rf "$ISO_DIR"/*
mkdir -p "$ISO_DIR/boot"

# Build Scala Native kernel
echo "== Building Scala Native kernel =="
( cd "$ROOT/kernel/scala" && sbt nativeLink )
cp "$ROOT/kernel/scala/target/scala-3.3.3/teletubbyos-out" "$ISO_DIR/boot/kernel.elf"

cp "$ROOT/boot/limine.cfg" "$ISO_DIR/boot/limine.cfg"

# Fetch limine if missing
LIMINE_DIR="$ROOT/tools/limine"
if [[ ! -d "$LIMINE_DIR" ]]; then
  mkdir -p "$ROOT/tools"
  git clone --depth=1 --branch v4.20231024.eol-binary \
    https://github.com/limine-bootloader/limine.git "$LIMINE_DIR"
fi

cp "$LIMINE_DIR/limine.sys"        "$ISO_DIR/boot/"
cp "$LIMINE_DIR/limine-cd.bin"     "$ISO_DIR/boot/"
cp "$LIMINE_DIR/limine-cd-efi.bin" "$ISO_DIR/boot/"

mkdir -p "$ISO_DIR/EFI/BOOT"
cp "$LIMINE_DIR/BOOTX64.EFI"  "$ISO_DIR/EFI/BOOT/"
cp "$LIMINE_DIR/BOOTIA32.EFI" "$ISO_DIR/EFI/BOOT/" 2>/dev/null || true

xorriso -as mkisofs \
  -b boot/limine-cd.bin \
  -no-emul-boot -boot-load-size 4 -boot-info-table \
  --efi-boot boot/limine-cd-efi.bin \
  -efi-boot-part --efi-boot-image --protective-msdos-label \
  "$ISO_DIR" -o "$ISO"

gcc -o "$LIMINE_DIR/limine-deploy" "$LIMINE_DIR/limine-deploy.c"
"$LIMINE_DIR/limine-deploy" "$ISO"

qemu-system-x86_64 \
  -M q35 \
  -m 256M \
  -cdrom "$ISO" \
  -serial stdio \
  -no-reboot -no-shutdown
