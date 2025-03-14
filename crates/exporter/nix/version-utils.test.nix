{
  pkgs ?
    import
      (builtins.fetchTarball "https://github.com/NixOS/nixpkgs/archive/refs/heads/nixos-unstable.tar.gz")
      { },
  lib ? pkgs.lib,
}:

let
  utils = import ./version-utils.nix { inherit lib; };

  assertValidStr = version: expected: {
    name = "isValidVersionStr(${version})";
    value = utils.isValidVersionStr version;
    expected = expected;
  };

  isVersionValidStrReport = [
    (assertValidStr "0.10.0-dev" true)
    (assertValidStr "0.10.0" true)
    (assertValidStr "0.10.1" true)
    (assertValidStr "0.10.100" true)
    (assertValidStr "0.11.0" true)

    (assertValidStr "x.x.x" true)
    (assertValidStr "0.x.x" true)
    (assertValidStr "0.10.x" true)
    (assertValidStr "^0.10.0" true)
    (assertValidStr "*" true)

    (assertValidStr "0.x.x.x" false)
    (assertValidStr "0.10" false)
    (assertValidStr "0.10." false)
  ];

  assertParseVersion =
    version: has_caret: has_greater_equals: major_base: major_must_equal: minor_base: minor_must_equal: patch_base: patch_must_equal: pre_release:
    let
      ex = {
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
      parsedVersion = (utils.parseVersion version);
    in
    {
      name = "parseVersion ${version}";
      value = parsedVersion == ex;
      expected = true;
    };
  parseVersionReport = [
    (assertParseVersion "0.10.0-dev" false false 0 true 10 true 0 true "-dev")
    (assertParseVersion "0.10.0" false false 0 true 10 true 0 true null)
    (assertParseVersion "0.10.1" false false 0 true 10 true 1 true null)
    (assertParseVersion "0.10.100" false false 0 true 10 true 100 true null)
    (assertParseVersion "0.11.0" false false 0 true 11 true 0 true null)

    (assertParseVersion "x.x.x" false false 0 false 0 false 0 false null)
    (assertParseVersion "0.x.x" false false 0 true 0 false 0 false null)
    (assertParseVersion "0.10.x" false false 0 true 10 true 0 false null)
    (assertParseVersion "^0.10.0" true false 0 true 10 true 0 true null)
    (assertParseVersion "^0.10.2" true false 0 true 10 true 2 true null)
    (assertParseVersion "^1.10.2" true false 1 true 10 true 2 true null)
    (assertParseVersion "*" false false 0 false 0 false 0 false null)

    (assertParseVersion ">=0.0.1" false true 0 true 0 true 1 true null)
    (assertParseVersion ">=2.4.3" false true 2 true 4 true 3 true null)
  ];

  reportExact =
    report:
    builtins.map (
      v:
      let
        status = if v.value == v.expected then "PASS" else "FAILED";
        body =
          if v.value == v.expected then
            ""
          else
            "expected: ${lib.boolToString v.expected} actually: ${lib.boolToString v.value}";
      in
      "${status} ${v.name} ${body}"
    ) report;

  testReport = {
    success = builtins.length (builtins.filter (v: v.value == v.expected) isVersionValidStrReport);
    failed = builtins.length (builtins.filter (v: v.value != v.expected) isVersionValidStrReport);
    report = (reportExact isVersionValidStrReport) ++ (reportExact parseVersionReport);
  };
in
{
  inherit testReport utils;
  isVersionValidStrRes = isVersionValidStrReport;
}
