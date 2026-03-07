{
  description = "A Telegram tag bot built with Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustVersion = pkgs.rust-bin.stable.latest.default;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        tagbot = rustPlatform.buildRustPackage {
          pname = "telegram-tag-bot";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ 
            pkgs.openssl
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          ];

          # We use bundled sqlite, so no need for system sqlite
          doCheck = false;
        };
      in
      {
        packages.default = tagbot;

        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustVersion
            pkgs.pkg-config
            pkgs.openssl
            pkgs.sqlite
          ];

          shellHook = ''
            export RUST_LOG=info
          '';
        };

        apps.default = flake-utils.lib.mkApp {
          drv = tagbot;
        };
      }
    );
}
