{
  pkgs ? import <nixpkgs> { },
  lib ? pkgs.lib,
}:

let
  VERSION_REGEX = "^(\\^|>=)?(([0-9]+)|x)\.(([0-9]+)|x)\.(([0-9]+)|x)(\-.*)?$";
  NOT_BEFORE_REGEXP = "^-([0-9]{4})([0-9]{2})([0-9]{2})$";

  elemAt =
    list: idx: default:
    if builtins.length list <= idx then
      default
    else
      let
        x = builtins.elemAt list idx;
      in
      if x == null then default else x;

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

  parseTime =
    dateTime:
    lib.strings.toInt (
      lib.readFile "${pkgs.runCommand "timestamp" { }
        "echo -n `${pkgs.coreutils}/bin/date -d ${dateTime} +%s` > $out"
      }"
    );

  # `version`: IParsedVersion
  normalizeVersion =
    version:
    let
      major_base = version.major_base;
      major_must_equal = version.major_must_equal;
      minor_base = version.minor_base;
      minor_must_equal = if version.has_caret && major_base != 0 then false else version.minor_must_equal;
      patch_base = version.patch_base;
      patch_must_equal = if version.has_caret then false else version.patch_must_equal;
      is_minimum = version.has_greater_equals;

      m_pre_release =
        if version.pre_release != null then
          (builtins.match NOT_BEFORE_REGEXP version.pre_release)
        else
          null;
      year = if m_pre_release == null then "1970" else elemAt m_pre_release 1 "1970";
      month = if m_pre_release == null then "01" else elemAt m_pre_release 2 "01";
      day = if m_pre_release == null then "01" else elemAt m_pre_release 3 "01";
      timestamp = parseTime "${year}-${month}-${day}T00:00:00Z";
      not_before = timestamp;
    in
    {
      inherit
        major_base
        major_must_equal
        minor_base
        minor_must_equal
        patch_base
        patch_must_equal
        is_minimum
        not_before
        ;
    }

  ;

  isValidVersion =
    version: product_ts: desired_version:

    let

      major_base = version.major_base;
      minor_base = version.minor_base;
      patch_base = version.patch_base;

      _desired_major_base = desired_version.major_base;
      _desired_minor_base = desired_version.minor_base;
      _desired_patch_base = desired_version.patch_base;
      desired_not_before = desired_version.not_before;

      _major_must_equal = desired_version.major_must_equal;
      _minor_must_equal = desired_version.minor_must_equal;
      _patch_must_equal = desired_version.patch_must_equal;
    in
    if desired_version.is_minimum then
      if major_base > _desired_major_base then
        true
      else

      if major_base < _desired_major_base then
        false
      else

      if minor_base > _desired_minor_base then
        true
      else

      if minor_base < _desired_minor_base then
        false
      else

      if product_ts != null && product_ts < desired_not_before then
        false
      else
        patch_base >= _desired_patch_base
    else
      let
        # Anything < 1.0.0 is compatible with >= 1.0.0, except exact matches
        compatible =
          major_base == 1
          && _desired_major_base == 0
          && (!_major_must_equal || !_minor_must_equal || !_patch_must_equal);

        desired_major_base = if compatible then 1 else _desired_major_base;
        desired_minor_base = if compatible then 0 else _desired_minor_base;
        desired_patch_base = if compatible then 0 else _desired_patch_base;
        major_must_equal = if compatible then true else _major_must_equal;
        minor_must_equal = if compatible then false else _minor_must_equal;
        patch_must_equal = if compatible then false else _patch_must_equal;
      in

      if major_base < desired_major_base then
        # smaller major version
        false
      else

      if major_base > desired_major_base then
        # higher major version
        !major_must_equal
      else

      # at this point, majorBase are equal

      if minor_base < desired_minor_base then
        # smaller minor version
        false
      else

      if minor_base > desired_minor_base then
        # higher minor version
        !minor_must_equal
      else

      # at this point, minorBase are equal

      if patch_base < desired_patch_base then
        # smaller patch version
        false
      else

      if patch_base > desired_patch_base then
        # higher patch version
        !patch_must_equal
      else

      # at this point, patchBase are equal

      if product_ts != null && product_ts < desired_not_before then
        false
      else
        true;
  isValidVersionAny =
    {
      version_s,
      desired_version_s,
      product_ts ? null,
    }:
    let
      version_p = parseVersion version_s;
      desired_version_p = parseVersion version_s;
    in
    # version = normalizeVersion parseVersion version_s;
    # desired_version = normalizeVersion parseVersion desired_version_s;
    if version_p == null || desired_version_p == null then
      false
    else
      let
        version = normalizeVersion version_p;
        desired_version = normalizeVersion desired_version_p;
      in
      isValidVersion version product_ts desired_version;
  normalizeVersionLessThan =
    l: r:

    if l.major_base < r.major_base then
      true
    else if l.major_base > r.major_base then
      false
    else if l.minor_base < r.minor_base then
      true
    else if l.minor_base > r.minor_base then
      false
    else if l.patch_base < r.patch_base then
      true
    else if l.patch_base > r.patch_base then
      false
    else
      false;

  versionLessThan =
    l_str: r_str:
    let
      lp = parseVersion l_str;
      rp = parseVersion r_str;
    in
    if lp == null || rp == null then
      false
    else
      let
        l = normalizeVersion lp;
        r = normalizeVersion rp;
      in
      normalizeVersionLessThan l r;
in
{
  inherit
    isValidVersionStr
    parseVersion
    normalizeVersion
    isValidVersion
    parseTime
    isValidVersionAny
    versionLessThan
    ;
}
