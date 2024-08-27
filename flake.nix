{
  description = "Build by nix";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, ... }@inputs:
    with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        cargo = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
        toolchain =
          (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
        RUST_VERSION = toolchain.toolchain.channel;
      in {
        devShells.default = with pkgs;
          mkShell {
            buildInputs = [ rust-bin.stable.${RUST_VERSION}.default ];
          };
        packages = {
          ${cargo.package.name} = pkgs.rustPlatform.buildRustPackage {
            pname = cargo.package.name;
            version = cargo.package.version;
            cargoLock.lockFile = ./Cargo.lock;
            src = pkgs.lib.cleanSource ./.;
          };
        };

        formatter = pkgs.nixfmt-classic;
      });
}
