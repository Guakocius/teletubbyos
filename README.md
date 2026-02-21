# TeletubbyOS

TeletubbyOS is a teaching and research operating system focused on safety,
determinism and user-restriction ("teletubby-safe computing").

The system follows a Linux-like philosophy but implements a capability-based
security model instead of traditional UNIX permissions.

The kernel is written in Rust.
The userland will later be implemented in Scala.

---

## Current Status

Milestone M0: Bootstrapping

The OS currently:
- builds a freestanding kernel
- boots in QEMU
- outputs text via serial (COM1)

---

## Requirements

You only need:

- git
- bash
- internet connection

No specific Linux distribution is required.

Supported:
- Arch Linux
- Ubuntu
- Debian
- Linux Mint
- Fedora (best effort)

---

## Setup

Run the bootstrap script:

```bash
    ./tools/bootstrap.sh
```

This will automatically install all required dependencies for your system
and prepare the Rust toolchain.

---

## Running

After that, run:

```bash
    ./tools/qemu/run.sh
```

If successful you will see:

**TeletubbyOS kernel: booted.**
**Status: Teletubbys still contained.**


Serial output is the primary debugging interface.
Framebuffer and graphics will be implemented later.

---

## Philosophy

The system is designed to be:

- capability secure
- immutable base system
- restricted user environment
- reproducible build

The user should not be able to destroy the system — even intentionally.


