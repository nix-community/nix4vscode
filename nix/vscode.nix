{
  pkgs ? import <nixpkgs> { },
  lib ? pkgs.lib,
}:

let
  utils = import ./version.nix { inherit pkgs lib; };
  applyDecorator =
    mktAttr: system:
    let
      name = "${mktAttr.mktplcRef.publisher}.${mktAttr.mktplcRef.name}";
    in
    if builtins.pathExists ./decorators/${name}.nix then
      let
        decorator = import ./decorators/${name}.nix {
          inherit pkgs lib system;
        };
      in
      lib.attrsets.recursiveUpdate mktAttr decorator
    else
      mktAttr;
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
      toml = import path;
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
      if !builtins.hasAttr "platform" ext then
        true
      else
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

  # Expand flatten key
  # "x.a.b" = v; => x.a.b = v;
  expandFlatKey =
    key: value:
    let
      parts = lib.strings.splitString "." key;
      build =
        path: val:
        if path == [ ] then
          val
        else
          { ${builtins.elemAt path 0} = build (lib.lists.sublist 1 (builtins.length path) path) val; };
    in
    build parts value;

  # Convert a {"a.x.b" = v;} to { a = { x = { b = v; };}; }
  expandFlattenMap =
    attrSet:
    let
      list = lib.attrsets.mapAttrsToList (k: v: expandFlatKey k v) attrSet;
      attrs = builtins.foldl' (l: r: lib.attrsets.recursiveUpdate l r) { } list;
    in
    attrs;

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
        vscode-utils.buildVscodeMarketplaceExtension (
          applyDecorator {
            vsix = fetchExtension value;
            mktplcRef = {
              name = value.name;
              publisher = value.publisher;
              version = value.version;
              sha256 = value.hash;
            };
          } system
        )
      ) infos;
    in
    expandFlattenMap exts;
in
{
  inherit
    infoFromFile
    extensionsFromInfo
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
      lib = {
        inherit
          infoExtensionForEngineList
          infoExtensionForEngineForSystem
          infoExtensionForEngineForSystemList
          infoExtensionForSystemList
          expandFlattenMap
          expandFlatKey
          ;
      };
    };
}
