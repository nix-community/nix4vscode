{
  pkgs ? import <nixpkgs> { },
  lib ? pkgs.lib,
}:

let
  applyDecorator =
    mktAttr: system:
    let
      name = "${mktAttr.mktplcRef.publisher}.${mktAttr.mktplcRef.name}";
      overName = "nix4vscode-${mktAttr.mktplcRef.publisher}.${mktAttr.mktplcRef.name}";
    in
    if builtins.hasAttr overName pkgs then
      lib.attrsets.recursiveUpdate mktAttr pkgs.${overName}
    else if builtins.pathExists ./decorators/${name}.nix then
      let
        decorator = import ./decorators/${name}.nix {
          inherit pkgs lib system;
        };
      in
      lib.attrsets.recursiveUpdate mktAttr decorator
    else
      mktAttr;

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
          url = info.u;
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
    extensionsFromInfo
    ;
}
