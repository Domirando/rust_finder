{ pkgs ? import <nixpkgs> { }, src ? ./. }:

let theSource = src; in
pkgs.rustPlatform.buildRustPackage rec {
  pname = "rust_finder";
  version = "0.1";
  src = pkgs.lib.cleanSource "${theSource}";
  cargoLock.lockFile = "${theSource}/Cargo.lock";
}