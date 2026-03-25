{
  description = "Psst development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };

        toolchain = fenix.packages.${system}.stable.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
          "rustfmt"
        ];

        libs = with pkgs; [
          openssl
          glib
          gtk3
          cairo
          pango
          atk
          gdk-pixbuf
          libsoup_3
          webkitgtk_4_1
          alsa-lib
          dbus
          fontconfig
          wayland
          libxkbcommon
          vulkan-loader
        ];
      in {
        devShells.default = pkgs.mkShell {
          buildInputs =
            [
              toolchain
              pkgs.pkg-config
            ]
            ++ libs;

          shellHook = ''
            export RUST_SRC_PATH="${toolchain}/lib/rustlib/src/rust/library"
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath libs}:$LD_LIBRARY_PATH"
          '';
        };
      }
    );
}
