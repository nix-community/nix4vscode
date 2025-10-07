# Nix4Vscode

nix4vscode is a Nix overlay for VSCode. It supports both VSCode and OpenVSX.

## Usage

nix4vscode supports VSCode by Nix overlays, here's how you can use it:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-24.11";

    nix4vscode = {
      url = "github:nix-community/nix4vscode";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { nixpkgs, nix4vscode, ... }:
    let
      pkgs = import nixpkgs {
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
        # ...
      ];
    in
    {
      # ... your flake outputs
    };
}
```

## Supported Functions

The overlay has exported the following functions:

### Basic Functions

| Usage                                                          | Description   |
| -------------------------------------------------------------- | ------------- |
| `forVscode [ extension ]`                                        | Install extensions from VSCode marketplace using default version |
| `forVscodeVersion version [ extension ]`                         | Install extensions for specific VSCode version |
| `forVscodePrerelease [ extension ]`                              | Install prerelease extensions from VSCode marketplace |
| `forVscodeVersionPrerelease version [ extension ]`               | Install prerelease extensions for specific VSCode version |
| `forOpenVsx [ extension ]`                                       | Install extensions from OpenVSX registry |
| `forOpenVsxVersion version [ extension ]`                        | Install extensions from OpenVSX for specific version |
| `forOpenVsxPrerelease [ extension ]`                             | Install prerelease extensions from OpenVSX |
| `forOpenVsxVersionPrerelease version [ extension ]`              | Install prerelease extensions from OpenVSX for specific version |

### Extended Functions with Custom Decorators

| Usage                                                          | Description   |
| -------------------------------------------------------------- | ------------- |
| `forVscodeExt decorators [ extension ]`                         | Install extensions with custom decorators |
| `forVscodeExtVersion decorators version [ extension ]`          | Install extensions with decorators for specific version |
| `forVscodeExtPrerelease decorators [ extension ]`               | Install prerelease extensions with decorators |
| `forVscodeExtVersionPrerelease decorators version [ extension ]` | Install prerelease extensions with decorators for specific version |
| `forOpenVsxExt decorators [ extension ]`                        | Install OpenVSX extensions with decorators |
| `forOpenVsxExtVersion decorators version [ extension ]`         | Install OpenVSX extensions with decorators for specific version |
| `forOpenVsxExtPrerelease decorators [ extension ]`              | Install prerelease OpenVSX extensions with decorators |
| `forOpenVsxExtVersionPrerelease decorators version [ extension ]` | Install prerelease OpenVSX extensions with decorators for specific version |

### Parameters

* `version`: string - VSCode version (e.g., "1.100.2")
* `decorators`: attribute set - Custom decorators for extensions
* `[ extension ]`: list of strings - Extension identifiers
    * type extension = "ms-vscode.cpptools" | "ms-vscode.cpptools.1.8.2"

### Custom Decorators

Decorators allow you to customize extension installation with patches, dependencies, or other modifications:

```nix
let
  myDecorators = {
    "ms-vscode.cpptools" = {
      postPatch = ''
        echo "Applying custom patch"
        # Custom installation logic
      '';
      buildInputs = [ pkgs.clang-tools ];
    };

    "rust-lang.rust-analyzer" =
      {
        pkgs,
        lib,
        system,
      }:
      {
        # Dynamic decorator using function
        buildInputs = [ pkgs.rust-analyzer ];
      };
  };
in
{
  extensions = pkgs.nix4vscode.forVscodeExt myDecorators [
    "ms-vscode.cpptools"
    "rust-lang.rust-analyzer"
  ];
}
```

Decorator priority (highest to lowest):
1. External decorators (passed via `decorators` parameter)
2. Package overrides (in `pkgs.nix4vscode-publisher.extension`)
3. Local decorator files (in `nix/decorators/`)

## Example

The [`example/`](./example/) directory provides a complete Home Manager configuration example that demonstrates how to use nix4vscode with various extensions from both VSCode Marketplace and OpenVSX registries. It includes examples of version-specific extensions, prerelease extensions, and custom decorators.
