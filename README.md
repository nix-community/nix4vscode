# Nix4Vscode

## Overlays

nix4vscode has **experimental** support for Nix overlays, here's how you can use it:

```nix
{
    inputs = {
        nixpkgs.url = "github:NixOS:nixpkgs/release-24.11";

        nix4vscode.url = {
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
                nix4vscode.overlays.forVscode
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
# `self` or `input` attrubute is the attribute you pass via `specialArgs` when you call `darwinSytem` or `nixosSytem`
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

### Supported function

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

## Fetcher

A tool generate nix expression from `config.toml`. Assuming we have a `config.toml` file like this:

```toml
vscode_version = "1.81.1"

extensions = [
    "eamodio.gitlens",
    "vscodevim.vim",
    { publisher_name = "ms-python", extension_name = "debugpy" },
]
```

We can then run `cargo run -- config.toml`, and a nix expression will be print to `stdout` just like this:

```nix
{ pkgs, lib }:

let
  vscode-utils = pkgs.vscode-utils;
in
{
  eamodio.gitlens = vscode-utils.extensionFromVscodeMarketplace {
    name = "gitlens";
    publisher = "eamodio";
    version = "2023.9.905";
    sha256 = "1mzyc3sinkg4zmbyh2a85iqdqa7wsnh99hqvk6f8m2jcfhpfrwyb";
  };
  vscodevim.vim = vscode-utils.extensionFromVscodeMarketplace {
    name = "vim";
    publisher = "vscodevim";
    version = "1.26.0";
    sha256 = "0hxb58ygjbqk6qmkp1r421zzib2r1vmz7agbi7bcmjxjpr3grw2w";
  };
}
```

Let's assume you store these contents in a file named `pkgs.nix`, you can use it by:

```nix
{ pkgs, lib }:
let
  plugins = (import ./vscode_plugins.nix) { inherit pkgs lib; };
in
with pkgs;
{
  enableExtensionUpdateCheck = false;
  enableUpdateCheck = false;
  extensions = with vscode-marketplace;[
    plugins.vscodevim.vim
  ]
  ;
}
```

### Installation

The simplest way to run nix4vscode is inside a devshell. Clone the nix4vscode repository, change into the resulting directory and run:

```shell
$ git clone https://github.com/nix-community/nix4vscode.git
$ cd nix4vscode
$ nix develop
$ cargo run -- config.toml
```

Replace `config.toml` with the name and path to your VSCode plugin configuration file.


### Creating the config.toml file

If you don't already have one, you can create the `config.toml` file by running the following script:

```shell
#!/bin/bash

# Output the VSCode version
echo 'vscode_version = "'$(code --version | head -n1)'"'
echo

# Loop through each installed extension
code --list-extensions | while read extension; do
  publisher_name=$(echo "$extension" | cut -d '.' -f 1)
  extension_name=$(echo "$extension" | cut -d '.' -f 2-)
  echo '[[extensions]]'
  echo 'publisher_name = "'$publisher_name'"'
  echo 'extension_name = "'$extension_name'"'
  echo
done
```

Just create the script, make it executable and then pipe the output to your `config.toml` file.
