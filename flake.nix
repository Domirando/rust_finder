{
  description = "Rust CLI tool using nixpkgs rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

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

        devShells.default = import ./shell.nix pkgs;
      });
}