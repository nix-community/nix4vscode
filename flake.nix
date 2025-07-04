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
      eachSystem = nixpkgs.lib.genAttrs (import systems);
      eachDefaultSystem =
        f:
        eachSystem (
          system:
          let
            pkgs = import nixpkgs {
              inherit system;
              overlays = [
                self.overlays.forVscode
              ];
            };
          in
          f system pkgs
        );

      lib = eachDefaultSystem (
        system: pkgs:
        let
          inherit (pkgs) lib;
          vscodePath = ./data/extensions.json;
          openVsxPath = ./data/extensions_openvsx.json;

          forVscodeVersionRaw =
            {
              dataPath ? vscodePath,
              version ? pkgs.vscode.version,
              extensions,
              pickPreRelease ? false,
              decorators ? null,
            }:
            let
              filters = builtins.map (v: ''--name="${v}"'') extensions;
              filter = builtins.concatStringsSep " " filters;
              prerelease = if pickPreRelease then "--prerelease" else "";
              mainTs = ./scripts/out.js;
              filteredExtensions = builtins.fromJSON (
                builtins.readFile (
                  pkgs.runCommandNoCC "nix4vscode-${version}" { } ''
                    ${pkgs.deno}/bin/deno run -A ${mainTs} --file ${dataPath} --engine ${version} --platform ${system} ${prerelease} --output=$out ${filter}
                  ''
                )
              );
              vscode = import ./nix/vscode.nix {
                inherit pkgs;
              };
              vscode-marketplace = vscode.extensionsFromInfo {
                inherit system decorators;
                extensions = filteredExtensions;
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
              ) extensions;
              attrs = (builtins.attrNames vscode-marketplace);
              diff = listDifference names (builtins.map (x: lib.strings.toLower x) attrs);

              validateAttribute =
                if builtins.length diff == 0 then
                  vscode-marketplace
                else
                  throw ''
                    The following extensions were not found: ${builtins.concatStringsSep "," diff}
                    1) Is there a spelling error? (Case insensitive)
                    2) Is there a version of the specified extension suitable for vscode `${version}`
                    3) If the specified extension has no stable version? If not, you may need forVscodePrerelease
                  '';
            in
            builtins.attrValues validateAttribute;

          generateSourceFunctions = rawFunction: {
            default = extensions: rawFunction { inherit extensions; };
            version = version: extensions: rawFunction { inherit version extensions; };
            prerelease =
              extensions:
              rawFunction {
                inherit extensions;
                pickPreRelease = true;
              };
            versionPrerelease =
              version: extensions:
              rawFunction {
                inherit version extensions;
                pickPreRelease = true;
              };
          };

          generateSourceFunctionsExt = rawFunction: {
            defaultExt = decorators: extensions: rawFunction { inherit extensions decorators; };
            versionExt =
              decorators: version: extensions:
              rawFunction { inherit version extensions decorators; };
            prereleaseExt =
              decorators: extensions:
              rawFunction {
                inherit extensions decorators;
                pickPreRelease = true;
              };
            versionPrereleaseExt =
              decorators: version: extensions:
              rawFunction {
                inherit version extensions decorators;
                pickPreRelease = true;
              };
          };

          vscodeVariants = generateSourceFunctions forVscodeVersionRaw;
          openvsxVariants = generateSourceFunctions (
            attr:
            forVscodeVersionRaw {
              dataPath = openVsxPath;
              version = pkgs.vscodium.version;
            }
            // attr
          );

          vscodeExtVariants = generateSourceFunctionsExt forVscodeVersionRaw;
          openvsxExtVariants = generateSourceFunctionsExt (
            attr:
            forVscodeVersionRaw {
              dataPath = openVsxPath;
              version = pkgs.vscodium.version;
            }
            // attr
          );

        in
        {
          forVscode = vscodeVariants.default;
          forVscodeVersion = vscodeVariants.version;
          forVscodePrerelease = vscodeVariants.prerelease;
          forVscodeVersionPrerelease = vscodeVariants.versionPrerelease;

          forOpenVsx = openvsxVariants.default;
          forOpenVsxVersion = openvsxVariants.version;
          forOpenVsxPrerelease = openvsxVariants.prerelease;
          forOpenVsxVersionPrerelease = openvsxVariants.versionPrerelease;

          forVscodeExt = vscodeExtVariants.defaultExt;
          forVscodeExtVersion = vscodeExtVariants.versionExt;
          forVscodeExtPrerelease = vscodeExtVariants.prereleaseExt;
          forVscodeExtVersionPrerelease = vscodeExtVariants.versionPrereleaseExt;

          forOpenVsxExt = openvsxExtVariants.defaultExt;
          forOpenVsxExtVersion = openvsxExtVariants.versionExt;
          forOpenVsxExtPrerelease = openvsxExtVariants.prereleaseExt;
          forOpenVsxExtVersionPrerelease = openvsxExtVariants.versionPrereleaseExt;

        }
      );
    in
    {
      inherit lib;
      devShells = eachDefaultSystem (
        system: pkgs: {
          default = pkgs.mkShell {
            strictDeps = true;
            packages = with pkgs; [
              esbuild
            ];
          };
        }
      );

      overlays = {
        default = (
          final: _: {
            nix4vscode = self.lib.${final.system};
          }
        );
        forVscode = (
          final: _: {
            nix4vscode = self.lib.${final.system};
          }
        );
      };

    };
}
