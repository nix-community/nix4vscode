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

      customeLib = lib.mapAttrs (
        system: pkgs:
        let
          forVscodeVersionRaw =
            engine: exts: pre_release:
            let
              filters = builtins.map (v: ''--name="${v}"'') exts;
              filter = builtins.concatStringsSep " " filters;
              prerelease = if pre_release then "--prerelease" else "";
              extensionPath = ./data/extensions.json;
              mainTs = ./scripts/out.js;
              extensions = builtins.fromJSON (
                builtins.readFile (
                  pkgs.runCommand "nix4vscode-${engine}" { } ''
                    ${pkgs.deno}/bin/deno run -A ${mainTs} --file ${extensionPath} --engine ${engine} --platform ${system} ${prerelease} --output=$out ${filter}
                  ''
                )
              );
              vscode = import ./nix/vscode.nix {
                pkgs = import nixpkgs {
                  inherit system;
                };
              };
              vscode-marketplace = vscode.extensionsFromInfo {
                inherit extensions system;
              };
              listDifference = a: b: builtins.filter (x: !(builtins.elem x b)) a;
              names = builtins.map (
                item:
                let
                  parts = lib.strings.splitString "." item;
                in
                if builtins.length parts < 3 then item else builtins.concatStringsSep "." (lib.lists.take 2 parts)
              ) exts;
              diff = listDifference names (builtins.attrNames vscode-marketplace);

              validateAttribute =
                if builtins.length diff == 0 then
                  vscode-marketplace
                else
                  throw "The folloing extension filed get: ${builtins.concatStringsSep "," diff}";
            in
            builtins.attrValues validateAttribute;

        in
        {
          forVscode = exts: forVscodeVersionRaw pkgs.vscode.version exts false;
          forVscodeVersion = version: exts: forVscodeVersionRaw version exts false;

          forVscodePrerelease = exts: forVscodeVersionRaw pkgs.vscode.version exts true;
          forVscodeVersionPrerelease = version: exts: forVscodeVersionRaw version exts true;
        }
      ) pkgsFor;
    in
    {
      lib = customeLib;
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
              # Use rustfmt, and other tools that require nightly features.
              (pkgs.rust-bin.selectLatestNightlyWith (
                toolchain:
                toolchain.minimal.override {
                  extensions = [
                    "rustfmt"
                    "rust-analyzer"
                  ];
                }
              ))
            ];
          };
        }
      ) pkgsFor;

      overlays = {
        default = lib.composeManyExtensions [ self.overlays.${packageName} ];
        forVscode = (
          final: _:
          let
            lib = {
              forVscode = customeLib.${final.system}.forVscode;
              forVscodeVersion = customeLib.${final.system}.forVscodeVersion;
              forVscodePrerelease = customeLib.${final.system}.forVscodePrerelease;
              forVscodeVersionPrerelease = customeLib.${final.system}.forVscodeVersionPrerelease;
            };
          in lib // { ${packageName} = lib; }
        );
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
    };
}
