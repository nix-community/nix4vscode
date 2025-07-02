{
  pkgs ?
    import
      (builtins.fetchTarball {
        url = "https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz";
      })
      {
        system = "aarch64-darwin";
      },
  lib ? pkgs.lib,
}:
let
  fetchurl = import ./fetchurl.nix {
    inherit pkgs lib;
  };
in
{
  inherit (fetchurl) fetchurl;
}
