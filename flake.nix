{
  description = "A tool to prefetch VS Code extensions for Nix expressions";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems = {
      url = "github:nix-systems/default";
      flake = false;
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
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
            extensionPath: engine: exts: pre_release:
            let
              filters = builtins.map (v: ''--name="${v}"'') exts;
              filter = builtins.concatStringsSep " " filters;
              prerelease = if pre_release then "--prerelease" else "";
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
                if builtins.length parts < 3 then
                  (lib.strings.toLower item)
                else
                  (lib.strings.toLower (builtins.concatStringsSep "." (lib.lists.take 2 parts)))
              ) exts;
              attrs = (builtins.attrNames vscode-marketplace);
              diff = listDifference names (builtins.map (x: lib.strings.toLower x) attrs);

              validateAttribute =
                if builtins.length diff == 0 then
                  vscode-marketplace
                else
                  throw "
The following extensions were not found: ${builtins.concatStringsSep "," diff}
1) Is there a spelling error? (Case insensitive)
2) Is there a version of the specified extension suitable for vscode `${engine}`
3) If the specified extension has no stable version? If not, you may need forVscodePrerelease
";
            in
            builtins.attrValues validateAttribute;

          vscodePath = ./data/extensions.json;
          openVsxPath = ./data/extensions_openvsx.json;

        in
        {
          forVscode = exts: forVscodeVersionRaw vscodePath pkgs.vscode.version exts false;
          forVscodeVersion = version: exts: forVscodeVersionRaw vscodePath version exts false;
          forVscodePrerelease = exts: forVscodeVersionRaw vscodePath pkgs.vscode.version exts true;
          forVscodeVersionPrerelease = version: exts: forVscodeVersionRaw vscodePath version exts true;

          forOpenVsx = exts: forVscodeVersionRaw openVsxPath pkgs.vscode.version exts false;
          forOpenVsxVersion = version: exts: forVscodeVersionRaw openVsxPath version exts false;
          forOpenVsxPrerelease = exts: forVscodeVersionRaw openVsxPath pkgs.vscode.version exts true;
          forOpenVsxVersionPrerelease = version: exts: forVscodeVersionRaw openVsxPath version exts true;
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
            packages = with pkgs; [
              esbuild
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
          final: _: {
            ${packageName} = customeLib.${final.system};
          }
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
