{
  pkgs ?
    import
      (builtins.fetchTarball "https://github.com/NixOS/nixpkgs/archive/refs/heads/nixos-unstable.tar.gz")
      { },
  lib ? pkgs.lib,
}:

let
  VERSION_REGEX = "^(\\^|>=)?(([0-9]+)|x)\.(([0-9]+)|x)\.(([0-9]+)|x)(\-.*)?$";
  NOT_BEFORE_REGEXP = "^-([0-9]{4})([0-9]{2})([0-9]{2})$";
  isValidVersionStr =
    version_s:
    let
      version = lib.strings.trim version_s;
    in
    version == "*" || builtins.match VERSION_REGEX version != null;

  parseVersion =
    version_s:
    let
      version = lib.strings.trim version_s;
      matches = builtins.match VERSION_REGEX version;
    in
    if isValidVersionStr version == false then
      null
    else if version == "*" then
      {
        has_caret = false;
        has_greater_equals = false;
        major_base = 0;
        major_must_equal = false;
        minor_base = 0;
        minor_must_equal = false;
        patch_base = 0;
        patch_must_equal = false;
        pre_release = null;
      }
    else
      let
        elemAt =
          list: idx: default:
          if builtins.length list <= idx then
            default
          else
            let
              x = builtins.elemAt list idx;
            in
            if x == null then default else x;

        prefix = elemAt matches 0 null;
        has_caret = prefix == "^";
        has_greater_equals = prefix == ">=";

        major_str = elemAt matches 2 "x";
        major_base = if major_str == "x" then 0 else lib.strings.toInt major_str;
        major_must_equal = major_str != "x";

        minor_str = elemAt matches 4 "x";
        minor_base = if minor_str == "x" then 0 else lib.strings.toInt minor_str;
        minor_must_equal = minor_str != "x";

        patch_str = elemAt matches 6 "x";
        patch_base = if patch_str == "x" then 0 else lib.strings.toInt patch_str;
        patch_must_equal = patch_str != "x";

        pre_release_str = elemAt matches 7 null;
        pre_release = pre_release_str;
      in
      {
        inherit
          has_caret
          has_greater_equals
          major_base
          major_must_equal
          minor_base
          minor_must_equal
          patch_base
          patch_must_equal
          pre_release
          ;
      };

in
{
  inherit isValidVersionStr parseVersion;
}
