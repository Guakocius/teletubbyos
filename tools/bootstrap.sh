#!/usr/bin/env bash
set -e

echo "== TeletubbyOS bootstrap starting =="

# Detect Nix
if command -v nix >/dev/null && [ -e /etc/NIXOS ]; then
    echo "NixOS detected."
    echo
    echo "Do NOT use bootstrap.sh on NixOS."
    echo "Use instead:"
    echo
    echo "    nix develop"
    echo "    ./tools/qemu/run.sh"
    echo
    exit 0
fi

if ! command -v git >/dev/null; then
    echo "git is required."
    exit 1
fi

# Detect distro
if [ -f /etc/arch-release ]; then
    DISTRO="arch"
elif grep -qi ubuntu /etc/os-release 2>/dev/null; then
    DISTRO="ubuntu"
elif grep -qi debian /etc/os-release 2>/dev/null; then
    DISTRO="debian"
elif grep -qi fedora /etc/os-release 2>/dev/null; then
    DISTRO="fedora"
else
    echo "Unsupported distribution. Please install manually:"
    echo "qemu-system-x86, xorriso, make, clang, lld"
    exit 1
fi

echo "Detected: $DISTRO"

install_arch() {
    sudo pacman -Sy --needed \
        qemu-system-x86 \
        xorriso \
        git \
        make \
        clang \
        lld
}

install_ubuntu_debian() {
    sudo apt update
    sudo apt install -y \
        qemu-system-x86 \
        xorriso \
        git \
        make \
        clang \
        lld \
        build-essential \
        curl
}

install_fedora() {
    sudo dnf install -y \
        qemu-system-x86 \
        xorriso \
        git \
        make \
        clang \
        lld \
        curl
}

case "$DISTRO" in
    arch) install_arch ;;
    ubuntu|debian) install_ubuntu_debian ;;
    fedora) install_fedora ;;
esac

# Rust installieren falls fehlt
if ! command -v rustup >/dev/null; then
    echo "Installing rustup..."
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    source "$HOME/.cargo/env"
fi

rustup toolchain install nightly
rustup component add rust-src llvm-tools-preview --toolchain nightly

echo "== Bootstrap finished successfully =="
