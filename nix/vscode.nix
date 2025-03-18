{
  pkgs ? import <nixpkgs> { },
  lib ? pkgs.lib,
}:

let
  utils = import ./version.nix { inherit pkgs lib; };
  # type Platfrom =
  # web
  # | linux-armhf
  # | alpine-arm64
  # | alpine-x64
  # | darwin-arm64
  # | win32-arm64
  # | win32-x64
  # | win32-ia32
  # | linux-x64
  # | linux-arm64
  # | darwin-x64

  # interface Extension {
  #  name: string;
  #  publisher: string;
  #  version: string;
  #  engine: string;
  #  platform: Platfrom;
  #  assert_url: string;
  #  hash: string;
  # }
  infoFromFile =
    path:
    let
      toml = builtins.fromTOML (builtins.readFile path);
    in
    if toml == null then null else toml.extension;

  infoExtensionForEngineList =
    extensions: engine:
    builtins.filter (
      ext:
      utils.isValidVersionAny {
        version_s = ext.version;
        desired_version_s = engine;
      }
    ) extensions;

  infoExtensionForSystemList =
    extensions: system:
    builtins.filter (
      ext:
      let
        plat =
          if system == "x86_64-linux" || system == "i686-linux" then
            [ "linux-x64" ]
          else if system == "aarch64-linux" then
            [ "linux-arm64" ]
          else if system == "armv7l-linux" then
            [ "linux-armhf" ]
          else if system == "x86_64-darwin" then
            [ "darwin-x64" ]
          else if system == "aarch64-darwin" then
            [ "darwin-arm64" ]
          else
            [ ];
      in
      builtins.elem ext.platform plat
    ) extensions;

  infoExtensionForEngineForSystemList =
    extensions: engine: system:
    infoExtensionForSystemList (infoExtensionForEngineList extensions engine) system;

  infoExtensionForEngineForSystem =
    extensions: engine: system:
    let
      exts = infoExtensionForEngineForSystemList extensions engine system;
      group = builtins.groupBy (el: "${el.publisher}.${el.name}") exts;
      maxV =
        li:
        builtins.foldl' (l: r: if (utils.versionLessThan l.version r.version) then r else l) {
          version = "0.0.0";
        } li;
    in
    builtins.mapAttrs (name: value: maxV (value)) group;

  extensionsFromInfo =
    {
      extensions,
      engine ? pkgs.vscode.version,
      system ? builtins.currentSystem,
    }:
    let
      infos = infoExtensionForEngineForSystem extensions engine system;
      vscode-utils = pkgs.vscode-utils;
      fetchExtension =
        info:
        pkgs.fetchurl {
          url = info.assert_url;
          name = "${info.publisher}-${info.name}.zip";
          sha256 = info.hash;
        };
      exts = builtins.mapAttrs (
        name: value:
        vscode-utils.buildVscodeMarketplaceExtension {
          vsix = fetchExtension value;
          mktplcRef = {
            name = value.name;
            publisher = value.publisher;
            version = value.version;
            sha256 = value.hash;
          };

        }
      ) infos;
    in
    exts;
in
{
  inherit
    infoExtensionForEngineList
    infoFromFile
    extensionsFromInfo
    infoExtensionForEngineForSystem
    infoExtensionForEngineForSystemList
    infoExtensionForSystemList
    ;

  debug =
    let

      forSystem =
        system:
        extensionsFromInfo {
          inherit system;
          extensions = infoFromFile ../data/extensions.toml;
        };
    in
    {
      inherit forSystem;
      dev = forSystem "aarch64-darwin";
    };
}
