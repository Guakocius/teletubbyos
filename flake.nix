{
  description = "TeletubbyOS development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          extensions = [ "rust-src" "llvm-tools-preview" ];
        });
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rust
            qemu
            xorriso
            git
            gnumake
            clang
            lld
            pkg-config
          ];

          shellHook = ''
            echo
            echo "TeletubbyOS Nix dev environment ready."
            echo "Run: ./tools/qemu/run.sh"
            echo
          '';
        };
      });
}
