# Nix4Vscode

A tool generate nix expression from `config.toml`. Assuming we have a `config.toml` file like this:

```toml
vscode_version = "1.81.1"

[[extensions]]
publisher_name = "eamodio"
extension_name = "gitlens"

[[extensions]]
publisher_name = "vscodevim"
extension_name = "vim"
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

## Redirect asset_url

For some extensions (such as codelldb), there is only a downloader in vscode markplace, and the real location of the extension is on github. At this time, asset_url is allowed to be redirected:

```toml
vscode_version = "1.81.1"

[[extensions]]
publisher_name = "vadimcn"
extension_name = "vscode-lldb"
asset_url = '''
https://github.com/vadimcn/codelldb/releases/download/v{{ extension.version }}/codelldb-{{ system.arch }}-{{ system.ostype }}.vsix
'''
```

asset_url is a jinja template string.

<!--
```nix
friendly-snippets = pkgs.vscode-utils.buildVscodeExtension {
  name = "friendly-snippets";
  vscodeExtPublisher = "rafamadriz";
  vscodeExtName = "friendly-snippets";
  src = (pkgs.fetchurl {
    url = "https://github.com/cathaysia/friendly-snippets/archive/refs/heads/version.zip";
    sha256 = "sha256-4TlMkVqaEgTO2kJrldJQl0MlZmF332ESarwoQpMylso=";
    name = "friendly-snippets.zip";
  }).outPath;
  vscodeExtUniqueId = "rafamadriz.friendly-snippets";
  version = "1.0.0";
}
```

```nix
vscode-utils.extensionsFromVscodeMarketplace [
  {
    name = "gitblame";
    publisher = "waderyan";
    version = "10.4.0";
    sha256 = "sha256-PPPlMGti+nRex6PBOxyu2qh6Rphl8kfdL9neNK1KkD0=";
  }
  {
    name = "python";
    publisher = "ms-python";
    version = "2023.15.12151010";
    sha256 = "sha256-gkQBAJudSUY19cCo0cD1uq61ZhtM/MeDz21k1LvNv64=";
  }
]
```

```nix
vscode-lldb = pkgs.vscode-utils.buildVscodeExtension {
  name = "vadimcn.vscode-lldb";
  vscodeExtPublisher = "vadimcn";
  vscodeExtName = "codelldb";
  src = (pkgs.fetchurl {
    url = "https://github.com/vadimcn/codelldb/releases/download/v1.9.2/codelldb-x86_64-linux.vsix";
    sha256 = "sha256-iYvSKyUFsSZx/ufS/hq7OE3GDRP1/sK0tlDQ2sP4PXU=";
    name = "codelldb.zip";
  }).outPath;
  vscodeExtUniqueId = "vadimcn.vscode-lldb";
  version = "1.9.2";
};
```
-->
