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
        inherit (pkgs.stdenv) isDarwin;

        rust_stable = pkgs.rust-bin.stable."1.76.0".default.override {
          extensions = [ "rust-src" "rust-analysis" "rust-std" "rust-docs" "clippy" ];
        };

      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.taplo
            rust_stable
          ] ++ lib.lists.optionals isDarwin [
            pkgs.iconv
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
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
