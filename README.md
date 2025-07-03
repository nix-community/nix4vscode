# Nix4Vscode

nix4vscode is a nix overlay for vscode. It supports both vscode and openvsx.

## Usage

nix4vscode support vscode by Nix overlays, here's how you can use it:

```nix
{
    inputs = {
        nixpkgs.url = "github:NixOS:nixpkgs/release-24.11";

        nix4vscode = {
            url = "github:nix-community/nix4vscode";
            inputs.nixpkgs.follows = "nixpkgs";
        };
    };

    outputs = { nixpkgs, nix4vscode ... }:
    let
        pkgs = import <nixpkgs> {
            config.allowUnfree = true;
            system = "aarch64-darwin"; # One of supported systems
            overlays = [
                nix4vscode.overlays.default
            ];
        };

        # Now you can access nix4vscode utilities from pkgs.nix4vscode
        extensions = pkgs.nix4vscode.forVscode [
            "tamasfe.even-better-toml"
            "ms-vscode-remote.remote-containers"
            # Using the specified version
            "editorconfig.editorconfig.0.9.4"
            /* ... */
        ];
    in
    {
        # ... your flake outputs
    };
}
```

Alternatively, if you use nix-darwin or NixOS you can add overlays to `nixpkgs` attribute in your one of modules:

```nix
# `self` or `input` attribute is the attribute you pass via `specialArgs` when you call `darwinSystem` or `nixosSystem`
{ self, ... }: {
    nixpkgs.overlays = [ self.inputs.nix4vscode.forVscode ];
}
```

Now, if you use VSCode with Home Manager, and you added overlays, you can install your extensions like this:

```nix
{ pkgs, ... }: {
    programs.vscode = {
        enable = true;
        enableUpdateCheck = false; # Disable VSCode self-update and let Home Manager to manage VSCode versions instead.
        enableExtensionUpdateCheck = false; # Disable extensions auto-update and let nix4vscode manage updates and extensions
        extensions = pkgs.nix4vscode.forVscode [
            "tamasfe.even-better-toml"
            "ms-vscode-remote.remote-containers.0.397.0" # You can also install specific version of extensions
        ];
    };
}
```

## Supported function

The Overlays has exported the following functions:

| Usage                                                          | description   |
| -------------------------------------------------------------- | ------------- |
| `forVscode [ extension ]`                                        | TODO          |
| `forVscodeVersion version [ extension ]`                         | TODO          |
| `forVscodePrerelease [ extension ]`                              | TODO          |
| `forVscodeVersionPrerelease version [ extension ]`               | TODO          |
| `forOpenVsx [ extension ]`                                       | TODO          |
| `forOpenVsxVersion version [ extension ]`                        | TODO          |
| `forOpenVsxPrerelease [ extension ]`                             | TODO          |
| `forOpenVsxVersionPrerelease version [ extension ]`              | TODO          |

* `version`: string
* `[ extension ]`:
    * type extension = "ms-vscode.cpptools" | "ms-vscode.cpptools.1.8.2"
