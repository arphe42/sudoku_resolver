{
  description = "rust environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
      rustVersion = pkgs.rust-bin.stable.latest.default;

      rustPlatform = pkgs.makeRustPlatform {
        cargo = rustVersion;
        rustc = rustVersion;
      };

      myRustBuild = rustPlatform.buildRustPackage {
        pname = "sudokuResolver";
        version = "0.1.0";
        src = ./.;

        cargoLock.lockFile = ./Cargo.lock;
      };

    in {
      packages = {
        rustPackage = myRustBuild;
      };
      defaultPackage = myRustBuild;
      devShell = pkgs.mkShell {
        buildInputs = 
        [ (rustVersion.override { extensions = [ "rust-src" ]; }) ];
      };
    });
}
