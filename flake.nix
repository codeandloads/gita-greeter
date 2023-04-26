{
  description = "gita_greeter flake";
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

        gita_greeter = rustPlatform.buildRustPackage {
          pname =
            "gita_greeter";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
        dockerImage = pkgs.dockerTools.buildImage {
          name = "gita_greeter";
          config = { cmd = [ "${gita_greeter}/bin/gita_greeter" ]; };
        };
      in
      {
        packages = {
          rustPackage = gita_greeter;
          docker = dockerImage;
        };
        defaultPackage = gita_greeter;
        devShell = pkgs.mkShell {
          buildInputs =
            [ (rustVersion.override { extensions = [ "rust-src" ]; }) ];
        };
      });
}
