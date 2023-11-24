{
  description = "nix4vscode test flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    nixpkgs-stable.url = "github:nixos/nixpkgs/22.11";
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
  };

  outputs = { self, ... }@inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
        plugins = (import ./vscode_plugins.nix) { pkgs = pkgs; lib = pkgs.lib; };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [
            plugins.eamodio.gitlens
            (builtins.getAttr "42crunch" plugins).vscode-openapi
          ];
        };
      }
    );
}
