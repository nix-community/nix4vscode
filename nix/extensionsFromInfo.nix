{
  decorators ? null,
  extensions,
  isOpenVsx ? false,
  lib ? pkgs.lib,
  pkgs ? import <nixpkgs> { },
  system ? pkgs.system,
}:

let
  getExtensionUrl = import ./getExtensionUrl.nix;
  applyDecorator =
    mktAttr: system: externalDecorators:
    let
      name = "${mktAttr.mktplcRef.publisher}.${mktAttr.mktplcRef.name}";
      overName = "nix4vscode-${mktAttr.mktplcRef.publisher}.${mktAttr.mktplcRef.name}";

      hasExternalDecorator = externalDecorators != null && builtins.hasAttr name externalDecorators;
      externalDecorator =
        if hasExternalDecorator then
          let
            decorator = externalDecorators.${name};
          in
          if builtins.isFunction decorator then decorator { inherit pkgs lib system; } else decorator
        else
          null;
    in
    if hasExternalDecorator then
      lib.attrsets.recursiveUpdate mktAttr externalDecorator
    else if builtins.hasAttr overName pkgs then
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
    let
      # infos = infoExtensionForEngineForSystem extensions engine system;
      vscode-utils = pkgs.callPackage ./vscode-utils {
      };
      fetchExtension =
        xname: info:
        let
          parts = lib.strings.splitString "." xname;
          publisher = builtins.elemAt parts 0;
          name = builtins.elemAt parts 1;
          url =
            info.u or (getExtensionUrl {
              inherit publisher name isOpenVsx;
              version = info.v;
              platform = info.p or null;
            });
        in
        builtins.fetchurl {
          url = url;
          name = "${publisher}-${name}.vsix";
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
        vscode-utils.buildVscodeMarketplaceExtension (applyDecorator attr system decorators)
      ) extensions;
    in
    exts;
in
extensionsFromInfo
