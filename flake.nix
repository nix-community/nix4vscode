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

  outputs = { self, nixpkgs, systems, rust-overlay }:
    let
      inherit (nixpkgs) lib;

      eachSystem = lib.genAttrs (import systems);
      pkgsFor = eachSystem (system:
        import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default self.overlays.default ];
        });

      cargoManifest = lib.importTOML ./Cargo.toml;
      packageName = cargoManifest.package.name;

      rustToolchain = lib.importTOML ./rust-toolchain.toml;
      rustVersion = rustToolchain.toolchain.channel;
    in {
      devShells = lib.mapAttrs (system: pkgs: {
        default = pkgs.mkShell {
          buildInputs = [ pkgs.rust-bin.stable.${rustVersion}.default ];
        };
      }) pkgsFor;

      overlays = {
        default = lib.composeManyExtensions [ self.overlays.${packageName} ];
        ${packageName} = final: _: {
          ${packageName} = final.rustPlatform.buildRustPackage {
            pname = packageName;
            version = cargoManifest.package.version;
            cargoLock.lockFile = ./Cargo.lock;
            src = lib.cleanSource ./.;
          };
        };
      };

      packages = lib.mapAttrs (system: pkgs: {
        default = self.packages.${system}.${packageName};
        ${packageName} = pkgs.${packageName};
      }) pkgsFor;

      formatter =
        eachSystem (system: nixpkgs.legacyPackages.${system}.nixfmt-classic);
    };
}
