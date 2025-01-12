# Nix4Vscode

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
  plugins = (import ./vscode_plugins.nix) { pkgs = pkgs; lib = lib; };
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

## Installation

The simplest way to run nix4vscode is inside a devshell. Clone the nix4vscode repository, change into the resulting directory and run:

```shell
$ git clone https://github.com/nix-community/nix4vscode.git
$ cd nix4vscode
$ nix develop
$ cargo run -- config.toml
```

Replace `config.toml` with the name and path to your VSCode plugin configuration file.

## Creating the config.toml file

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
