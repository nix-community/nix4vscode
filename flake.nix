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
                inherit system;
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

          forOpenVsxVersionRaw =
            attr:
            forVscodeVersionRaw {
              dataPath = openVsxPath;
              version = pkgs.vscodium.version;
            }
            // attr;

        in
        {
          forVscode = extensions: forVscodeVersionRaw { inherit extensions; };
          forVscodeVersion =
            version: extensions:
            forVscodeVersionRaw {
              inherit version extensions;
            };
          forVscodePrerelease =
            extensions:
            forVscodeVersionRaw {
              inherit extensions;
              pickPreRelease = true;
            };
          forVscodeVersionPrerelease =
            version: extensions:
            forVscodeVersionRaw {
              inherit version extensions;
              pickPreRelease = true;
            };

          forOpenVsx = extensions: forOpenVsxVersionRaw { inherit extensions; };
          forOpenVsxVersion = version: extensions: forOpenVsxVersionRaw { inherit extensions version; };
          forOpenVsxPrerelease =
            extensions:
            forOpenVsxVersionRaw {
              inherit extensions;
              pickPreRelease = true;
            };
          forOpenVsxVersionPrerelease =
            version: extensions:
            forOpenVsxVersionRaw {
              inherit version extensions;
              pickPreRelease = true;
            };

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
