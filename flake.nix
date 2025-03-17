{
  description = "A tool to prefetch VS Code extensions for Nix expressions";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems = {
      url = "github:nix-systems/default";
      flake = false;
    };
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      systems,
      rust-overlay,
    }:
    let
      inherit (nixpkgs) lib;

      vscode = import ./nix/vscode.nix {
        pkgs = (import nixpkgs { });
      };

      extensions = vscode.infoFromFile ./data/extensions.toml;
      eachSystem = lib.genAttrs (import systems);

      pkgsFor = eachSystem (
        system:
        import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
            self.overlays.default
          ];
        }
      );

      packageName = (lib.importTOML ./Cargo.toml).package.name;

      rustToolchain = lib.importTOML ./rust-toolchain.toml;
      rustVersion = rustToolchain.toolchain.channel;
    in
    {
      devShells = lib.mapAttrs (
        system: pkgs:
        let
          rust-stable = pkgs.rust-bin.stable.${rustVersion}.minimal.override {
            extensions = [
              "rust-src"
              "rust-docs"
              "clippy"
            ];
          };
        in
        {
          default = pkgs.mkShell {
            strictDeps = true;
            packages = [
              (lib.hiPrio rust-stable)
            ];
          };
        }
      ) pkgsFor;

      overlays = {
        default = lib.composeManyExtensions [ self.overlays.${packageName} ];
        ${packageName} =
          final: _:
          let
            rust-bin = rust-overlay.lib.mkRustBin { } final;
            rust-stable = rust-bin.stable.${rustVersion}.minimal;
            rustPlatform = final.makeRustPlatform {
              cargo = rust-stable;
              rustc = rust-stable;
            };
          in
          {
            ${packageName} = final.callPackage ./nix/package.nix {
              sourceRoot = self;
              inherit rustPlatform;
            };
          };
      };

      packages = lib.mapAttrs (system: pkgs: {
        default = self.packages.${system}.${packageName};
        ${packageName} = pkgs.${packageName};
      }) pkgsFor;

      formatter = eachSystem (system: nixpkgs.legacyPackages.${system}.nixfmt-classic);

      lib = eachSystem (system: {
        extensions = vscode.extensionsFromInfo {
          inherit extensions;
          platform = system;
        };

        extensionsFromInfo =
          engine:
          vscode.extensionsFromInfo {
            inherit extensions engine;
            platform = system;
          };
      });
    };
}
