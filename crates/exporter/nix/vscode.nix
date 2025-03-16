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
  fromFile =
    path:
    let
      toml = builtins.fromTOML (builtins.readFile path);
    in
    if toml == null then null else toml.extension;

  extensionForEngineList =
    extensions: engine:
    builtins.filter (
      ext:
      utils.isValidVersionAny {
        version_s = ext.version;
        desired_version_s = engine;
      }
    ) extensions;

  extensionForPlatformList =
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

  extensionForEngineForPlatformList =
    extensions: engine: platform:
    extensionForPlatformList (extensionForEngineList extensions engine) platform;

  extensionForEngineForPlatform =
    extensions: engine: platform:
    let
      li= extensionForEngineForPlatformList extensions engine platform;
    in
    { };

in
{
  inherit
    fromFile
    extensionForEngineList
    extensionForPlatformList
    extensionForEngineForPlatformList
    extensionForEngineForPlatform
    ;

  x = extensionForEngineForPlatformList (fromFile ./extensions.toml) "1.76.0" "x86_64-linux";
}
