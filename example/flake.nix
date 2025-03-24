{
  description = "nix4vscode test flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    nixpkgs-stable.url = "github:nixos/nixpkgs/22.11";
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    nix4vscode = {
      url = "..";
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
        };
        vscode-marketplace = nix4vscode.lib.${system}.forVscode [
          "ms-vscode.cpptools"
        ];
        plugins = import ./vscode_plugins.nix {
          inherit pkgs;
          lib = pkgs.lib;
        };
      in
      {
        inherit vscode-marketplace;
        devShell = pkgs.mkShell {
          buildInputs = [
            plugins.eamodio.gitlens

            (builtins.getAttr "42crunch" plugins).vscode-openapi
            plugins.ms-vscode.cpptools
          ] ++ vscode-marketplace;
        };
      }
    );
}
