# Nix4Vscode

A tool generate nix expression from config.toml. Let's guess we have a config.toml like this:

```toml
vscode_version = "1.81.1"

[[extensions]]
publisher_name = "eamodio"
extension_name = "gitlens"

[[extensions]]
publisher_name = "vscodevim"
extension_name = "vim"

[[extensions]]
publisher_name = "vadimcn"
extension_name = "vscode-lldb"
```

then run `cargo run -- -f config.toml` , the a nix expression will be print to stdout just like this:

```nix
{ pkgs, lib }:

let
  vscode-utils = pkgs.vscode-utils;
in
[
  (vscode-utils.buildVscodeExtension {
    name = "vadimcn.vscode-lldb";
    vscodeExtPublisher = "vadimcn";
    vscodeExtName = "vscode-lldb";
    src = (pkgs.fetchurl {
      url = "https://github.com/vadimcn/codelldb/releases/download/v1.9.2/codelldb-x86_64-linux.vsix";
      sha256 = "0x9xz31xml2hnssc5zpm2c6wck9qpcdgxlp7zrqjdc854lmx52w9";
      name = "vadimcn.vscode-lldb.zip";
    }).outPath;
    vscodeExtUniqueId = "vadimcn.vscode-lldb";
    version = "";
  })
] ++
vscode-utils.extensionsFromVscodeMarketplace [
  {
    name = "gitlens";
    publisher = "eamodio";
    version = "2023.9.905";
    sha256 = "1mzyc3sinkg4zmbyh2a85iqdqa7wsnh99hqvk6f8m2jcfhpfrwyb";
  }
  {
    name = "vim";
    publisher = "vscodevim";
    version = "1.25.2";
    sha256 = "0j0li3ddrknh34k2w2f13j4x8s0lb9gsmq7pxaldhwqimarqlbc7";
  }
]
```

Let's guess you store those contents in a file named `pkgs.nix`, then you can use it by:

```nix
{ pkgs, lib }:
let
  inherit (pkgs.stdenv) isDarwin isLinux;
in
with pkgs;
{
  enableExtensionUpdateCheck = false;
  enableUpdateCheck = false;
  extensions = with vscode-marketplace;[
  ]
  ] ++ (import ./pkgs.nix) { pkgs = pkgs; lib = lib; }
  ;
}
```

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
