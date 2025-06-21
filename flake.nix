{
  description = "Rust CLI tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rust = pkgs.rust-bin.stable.latest.default;

        rust_finder = pkgs.rustPlatform.buildRustPackage {
          pname = "rust_finder";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
      in {
        packages.default = rust_finder;

        apps.default = flake-utils.lib.mkApp {
          drv = rust_finder;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [ rust ];
        };
      });
}