{
  description = "Home Manager configuration of nix4vscode";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nix4vscode = {
      url = "../";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
  };

  outputs =
    { self, ... }@inputs:
    with inputs;
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
          overlays = [
            nix4vscode.overlays.default
          ];
        };

      in
      {
        packages = {
          homeConfigurations.root = home-manager.lib.homeManagerConfiguration {
            inherit pkgs;

            modules = [
              ./home.nix
            ];
          };
        };
      }
    );
}
