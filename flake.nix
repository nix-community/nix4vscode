{
  description = "Develop environment for nix4vscode";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
          overlays = [
            (import rust-overlay)
          ];
        };

      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.iconv
            pkgs.taplo
            (
              pkgs.rust-bin.nightly."2023-07-28".default
            )
          ];
          shellHook = ''
            export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.iconv.out}/lib
          '';
        };
      });
}
