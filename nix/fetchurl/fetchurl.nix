{
  pkgs,
  lib ? pkgs.lib,
}:

let
  fetchurlAxel =
    {
      url,
      sha256 ? "",
      name ? "",
      outputHash ? sha256,
      outputHashAlgo ? "sha256",
      ...
    }@args:
    let
      finalName = if name != "" then name else baseNameOf url;

      # Remove fetchurl-specific args that stdenv.mkDerivation doesn't understand
      cleanArgs = builtins.removeAttrs args [
        "sha256"
        "name"
        "outputHash"
        "outputHashAlgo"
      ];
    in
    pkgs.stdenvNoCC.mkDerivation (
      cleanArgs
      // {
        name = finalName;
        builder = ./builder.sh;
        nativeBuildInputs = [ pkgs.axel ];

        outputHashMode = "flat";
        inherit outputHash outputHashAlgo;

        meta = {
          description = "Download ${url} using axel";
          platforms = lib.platforms.all;
        };
      }
    );

in
{
  inherit fetchurlAxel;

  # Provide a drop-in replacement for fetchurl
  fetchurl = fetchurlAxel;
}
