{
  description = "A tool to prefetch VS Code extensions for Nix expressions";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems = {
      url = "github:nix-systems/default";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      systems,
    }:
    let
      inherit (nixpkgs) lib;

      eachSystem = lib.genAttrs (import systems);
      pkgsFor = eachSystem (
        system:
        import nixpkgs {
          inherit system;
          overlays = [
            self.overlays.forVscode
          ];
        }
      );

      customLib = lib.mapAttrs (
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
                  pkgs.runCommandNoCC "nix4vscode-${engine}" { } ''
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
      lib = customLib;
      devShells = lib.mapAttrs (system: pkgs: {
        default = pkgs.mkShell {
          strictDeps = true;
          packages = with pkgs; [
            esbuild
          ];
        };
      }) pkgsFor;

      overlays = {
        default = (
          final: _: {
            nix4vscode = customLib.${final.system};
          }
        );
        forVscode = (
          final: _: {
            nix4vscode = customLib.${final.system};
          }
        );
      };

    };
}
