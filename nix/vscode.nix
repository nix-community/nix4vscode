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
  #  url: string;
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
      if !builtins.hasAttr "p" ext then
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
        builtins.elem ext.p plat
    ) extensions;

  infoExtensionForEngineForSystemList =
    extensions: engine: system:
    infoExtensionForSystemList (infoExtensionForEngineList extensions engine) system;

  infoExtensionForEngineForSystem =
    extensions: engine: system:
    let
      group = extensions;
      maxV =
        li:
        builtins.foldl' (l: r: if (utils.versionLessThan l.v r.v) then r else l) {
          v = "0.0.0";
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
      # infos = infoExtensionForEngineForSystem extensions engine system;
      vscode-utils = pkgs.vscode-utils;
      fetchExtension =
        xname: info:
        let
          parts = lib.strings.splitString "." xname;
          publisher = builtins.elemAt parts 0;
          name = builtins.elemAt parts 1;
          url =
            if builtins.match "[0-9]+" info.u == null then
              info.u
            else
              "https://${publisher}.gallerycdn.vsassets.io/extensions/${publisher}/${name}/${info.v}/${info.u}/Microsoft.VisualStudio.Services.VSIXPackage";
        in
        pkgs.fetchurl {
          url = url;
          name = "${publisher}-${name}.zip";
          sha256 = info.h;
        };
      exts = builtins.mapAttrs (
        xname: value:
        let
          parts = lib.strings.splitString "." xname;
          publisher = builtins.elemAt parts 0;
          name = builtins.elemAt parts 1;
          attr = {
            vsix = fetchExtension xname value;
            mktplcRef = {
              inherit name publisher;
              version = value.v;
              sha256 = value.h;
            };
          };
        in
        vscode-utils.buildVscodeMarketplaceExtension (applyDecorator attr system)
      ) extensions;
    in
    exts;
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
