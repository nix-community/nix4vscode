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
        lib = pkgs.lib;
        inherit (pkgs.stdenv) isDarwin isLinux;

        rust_toolchain = {
          extensions = [ "rust-src" "rust-analysis" "rust-std" "rust-docs" "clippy" "rust-analyzer" "llvm-tools-preview" ];
        };

      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.taplo
            (
              pkgs.rust-bin.nightly."2023-07-28".default.override rust_toolchain
            )
          ] ++ lib.lists.optionals isDarwin [
            pkgs.iconv
            pkgs.darwin.apple_sdk.frameworks.Security
          ];
          shellHook = ''
            export DYLD_FALLBACK_LIBRARY_PATH=$(rustc --print sysroot)/lib
          '' + (if
            isDarwin == true then ''
            export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.iconv.out}/lib
          '' else '' '');
        };
      });
}


