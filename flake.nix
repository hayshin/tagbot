{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk-package = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, naersk-package, ... }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
      naersk = pkgs.callPackage naersk-package {};
      tagbot = naersk.buildPackage {
        src = ./.;
      };
    in {
      packages = {
        default = tagbot;
        dockerImage = pkgs.dockerTools.buildLayeredImage {
          name = "tagbot";
          tag = "latest";
          contents = [ pkgs.cacert ];
          config = {
            Cmd = [ "${tagbot}/bin/tagbot" ];
            Env = [ "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt" ];
          };
        };
      };
      defaultPackage = self.packages.${system}.default;
    });
}
