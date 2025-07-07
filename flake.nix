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
          forVscodeVersionRaw = import ./nix/forVscodeVersionRaw.nix;

          generateSourceFunctions = rawFunction: {
            default = extensions: rawFunction { inherit extensions pkgs; };
            version = version: extensions: rawFunction { inherit version extensions pkgs; };
            prerelease =
              extensions:
              rawFunction {
                inherit extensions pkgs;
                pickPreRelease = true;
              };
            versionPrerelease =
              version: extensions:
              rawFunction {
                inherit version extensions pkgs;
                pickPreRelease = true;
              };
          };

          generateSourceFunctionsExt = rawFunction: {
            defaultExt = decorators: extensions: rawFunction { inherit extensions decorators pkgs; };
            versionExt =
              decorators: version: extensions:
              rawFunction {
                inherit
                  version
                  extensions
                  decorators
                  pkgs
                  ;
              };
            prereleaseExt =
              decorators: extensions:
              rawFunction {
                inherit extensions decorators pkgs;
                pickPreRelease = true;
              };
            versionPrereleaseExt =
              decorators: version: extensions:
              rawFunction {
                inherit
                  version
                  extensions
                  decorators
                  pkgs
                  ;
                pickPreRelease = true;
              };
          };

          vscodeVariants = generateSourceFunctions forVscodeVersionRaw;
          openvsxVariants = generateSourceFunctions (
            attr:
            forVscodeVersionRaw {
              isOpenVsx = true;
              version = pkgs.vscodium.version;
            }
            // attr
          );

          vscodeExtVariants = generateSourceFunctionsExt forVscodeVersionRaw;
          openvsxExtVariants = generateSourceFunctionsExt (
            attr:
            forVscodeVersionRaw {
              isOpenVsx = true;
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
              nixfmt-rfc-style
              rustfmt
              taplo
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
