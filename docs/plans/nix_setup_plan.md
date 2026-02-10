# Nix Development Environment Setup

This plan outlines the creation of a reproducible Nix development environment for the `psst` project, including Rust tooling and system dependencies required for GUI development.

## User Review Required

> [!IMPORTANT]
> This setup introduces a `flake.nix` file to the repository root. Ensure you have Nix installed with flakes enabled.

## Proposed Changes

### [Root]

#### [NEW] [flake.nix](file:///home/icarus/dev/projects/psst/flake.nix)
- Defines a devShell using `fenix` for the Rust toolchain (latest stable).
- Includes system dependencies:
    - `pkg-config`, `openssl`
    - GUI libs: `gtk3`, `glib`, `cairo`, `pango`, `atk`, `gdk-pixbuf`, `libsoup`
    - Wayland/Vulkan support: `wayland`, `libxkbcommon`, `vulkan-loader`
    - Audio: `alsa-lib`, `dbus`

#### [NEW] [.envrc](file:///home/icarus/dev/projects/psst/.envrc)
- Enables `direnv` with `use flake`.

#### [MODIFY] [.gitignore](file:///home/icarus/dev/projects/psst/.gitignore)
- Adds `.direnv` to ignore list.

## Verification Plan

### Automated Tests
- Run `nix develop --command cargo --version` to verify Rust toolchain.
- Run `nix develop --command pkg-config --list-all | grep gtk` to verify system deps.
