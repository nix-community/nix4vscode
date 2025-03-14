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
      supportedSystems = import systems;
      forEachSystem = nixpkgs.lib.genAttrs supportedSystems;

      engineOverlay =
        lib:
        import ./nix/vscode-engine-overlay.nix {
          lib = lib;
        };
    in
    {
      overlays.default = engineOverlay nixpkgs.lib;

      # Provide a package with overlay for each supported system
      packages = forEachSystem (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (engineOverlay nixpkgs.lib) ];
          };
        in
        {
          default = pkgs.callPackage (
            { }:
            {
              inherit (pkgs) vscodeExtensionsForEngine;
            }
          ) { };
        }
      );

      # Provide a development environment with the overlay
      devShells = forEachSystem (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (engineOverlay nixpkgs.lib) ];
          };

          # Import the version utils test
          versionTests = import ./nix/version-utils-test.nix { inherit pkgs; };
        in
        {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              nixfmt-rfc-style
              versionTests.runTests
            ];

            # Provide vscodeExtensionsForEngine function in the shell
            shellHook = ''
              echo "VSCode Engine Overlay Development Shell"
              echo "Usage:"
              echo "  vscodeExtensionsForEngine <engine-version>"
              echo "  vscodeExtensionsForEnginePlatform <engine-version> <platform>"
              echo "  run-version-utils-tests (run version utility tests)"
              echo "Examples:"
              echo "  vscodeExtensionsForEngine 1.85.0"
              echo "  vscodeExtensionsForEnginePlatform 1.85.0 linux-x64"
              echo ""
              echo "Available platforms: web, alpine-arm64, linux-armhf, alpine-x64, darwin-arm64, linux-x64, linux-arm64, darwin-x64"

              vscodeExtensionsForEngine() {
                nix eval --raw -f ${./nix/example.nix} --apply "extensions: builtins.toJSON (extensions.\"$1\" or extensions)" | jq
              }

              vscodeExtensionsForEnginePlatform() {
                if [ $# -ne 2 ]; then
                  echo "Error: Two parameters required <engine-version> <platform>"
                  return 1
                fi

                # Use lib.vscodeExtensionsForEnginePlatform function
                nix eval --raw --impure --expr "
                  let
                    flake = builtins.getFlake (toString ./.);
                    extensions = flake.lib.vscodeExtensionsForEnginePlatform \"$1\" \"$2\";
                    formatExt = ext: {
                      name = \"''${ext.publisher}.''${ext.name}\";
                      version = ext.version;
                      engine = ext.engine;
                      platform = ext.platform;
                    };
                  in
                    builtins.toJSON (map formatExt extensions)
                " | jq
              }

              export -f vscodeExtensionsForEngine
              export -f vscodeExtensionsForEnginePlatform
            '';
          };
        }
      );

      # Provide a simple usage example
      lib = {
        vscodeExtensionsForEngine =
          engineVersion: ((engineOverlay nixpkgs.lib) null null).vscodeExtensionsForEngine engineVersion;

        vscodeExtensionsForEnginePlatform =
          engineVersion: platform:
          ((engineOverlay nixpkgs.lib) null null).vscodeExtensionsForEnginePlatform engineVersion platform;
      };
    };
}
