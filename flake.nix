{
  description = "Build by nix";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems = {
      url = "github:nix-systems/default";
      flake = false;
    };
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, ... }@inputs:
    with inputs;
    let
      inherit (nixpkgs) lib;

      eachSystem = lib.genAttrs (import systems);
      pkgsFor = eachSystem (system:
        import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        });

      cargoManifest = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
      rustToolchain =
        (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
      rustVersion = rustToolchain.toolchain.channel;
    in {
      devShells = lib.mapAttrs (system: pkgs: {
        default = pkgs.mkShell {
          buildInputs = [ pkgs.rust-bin.stable.${rustVersion}.default ];
        };
      }) pkgsFor;

      packages = lib.mapAttrs (system: pkgs: {
        ${cargoManifest.package.name} = pkgs.rustPlatform.buildRustPackage {
          pname = cargoManifest.package.name;
          version = cargoManifest.package.version;
          cargoLock.lockFile = ./Cargo.lock;
          src = lib.cleanSource ./.;
        };
      }) pkgsFor;

      formatter =
        eachSystem (system: nixpkgs.legacyPackages.${system}.nixfmt-classic);
    };
}
