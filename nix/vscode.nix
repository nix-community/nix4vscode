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

  infoExtensionForPlatformList =
    extensions: platform:
    builtins.filter (
      ext:
      let
        plat =
          if platform == "x86_64-linux" || platform == "i686-linux" then
            [ "linux-x64" ]
          else if platform == "aarch64-linux" then
            [ "linux-arm64" ]
          else if platform == "armv7l-linux" then
            [ "linux-armhf" ]
          else if platform == "x86_64-darwin" then
            [ "darwin-x64" ]
          else if platform == "aarch64-darwin" then
            [ "darwin-arm64" ]
          else
            [ ];
      in
      builtins.elem ext.platform plat
    ) extensions;

  infoExtensionForEngineForPlatformList =
    extensions: engine: platform:
    infoExtensionForPlatformList (infoExtensionForEngineList extensions engine) platform;

  infoExtensionForEngineForPlatform =
    extensions: engine: platform:
    let
      exts = infoExtensionForEngineForPlatformList extensions engine platform;
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
      platform ? builtins.currentSystem,
    }:
    let
      infos = infoExtensionForEngineForPlatform extensions engine platform;
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
    infoExtensionForEngineForPlatform
    infoExtensionForEngineForPlatformList
    infoExtensionForPlatformList
    infoExtensionForEngineList
    infoFromFile
    extensionsFromInfo
    ;

  x = extensionsFromInfo {
    extensions = (infoFromFile ../extensions.toml);
  };
}
