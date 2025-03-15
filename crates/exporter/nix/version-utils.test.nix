{
  pkgs ? import <nixpkgs> { },
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

  assertNormalizeVersion =
    version_s: major_base: major_must_equal: minor_base: minor_must_equal: patch_base: patch_must_equal: is_minimum: not_before:
    let
      ex = {
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
      };
      version = utils.parseVersion version_s;
      parsedVersion = (utils.normalizeVersion version);
    in
    {
      name = "normalizeVersion ${version_s}";
      value = parsedVersion == ex;
      expected = true;
    };
  normalizeVersionReport = [

    (assertNormalizeVersion "0.10.0-dev" 0 true 10 true 0 true false 0)
    (assertNormalizeVersion "0.10.0-222222222" 0 true 10 true 0 true false 0)
    # assertNormalizeVersion("0.10.0-20210511"  0  true  10  true  0  true  false  new Date('2021-05-11T00:00:00Z').getTime());

    (assertNormalizeVersion "0.10.0" 0 true 10 true 0 true false 0)
    (assertNormalizeVersion "0.10.1" 0 true 10 true 1 true false 0)
    (assertNormalizeVersion "0.10.100" 0 true 10 true 100 true false 0)
    (assertNormalizeVersion "0.11.0" 0 true 11 true 0 true false 0)

    (assertNormalizeVersion "x.x.x" 0 false 0 false 0 false false 0)
    (assertNormalizeVersion "0.x.x" 0 true 0 false 0 false false 0)
    (assertNormalizeVersion "0.10.x" 0 true 10 true 0 false false 0)
    (assertNormalizeVersion "^0.10.0" 0 true 10 true 0 false false 0)
    (assertNormalizeVersion "^0.10.2" 0 true 10 true 2 false false 0)
    (assertNormalizeVersion "^1.10.2" 1 true 10 false 2 false false 0)
    (assertNormalizeVersion "*" 0 false 0 false 0 false false 0)

    (assertNormalizeVersion ">=0.0.1" 0 true 0 true 1 true true 0)
    (assertNormalizeVersion ">=2.4.3" 2 true 4 true 3 true true 0)
    (assertNormalizeVersion ">=2.4.3" 2 true 4 true 3 true true 0)
  ];

  PRODUCT_VERSION = utils.parseTime "2021-05-11T21:54:30.577Z";
  assertIsValidVersion =
    version: desired_version: expected_result:
    let
      parseVersion = version: utils.normalizeVersion (utils.parseVersion version);

      actually = utils.isValidVersion (parseVersion version) PRODUCT_VERSION (
        parseVersion desired_version
      );
    in
    {
      name = "(isValidVersion ${version} ${desired_version}) == ${lib.boolToString expected_result}";
      value = actually;
      expected = expected_result;
    };

  isValidVersionReport = [
    (assertIsValidVersion "0.10.0-dev" "x.x.x" true)
    (assertIsValidVersion "0.10.0-dev" "0.x.x" true)
    (assertIsValidVersion "0.10.0-dev" "0.10.0" true)
    (assertIsValidVersion "0.10.0-dev" "0.10.2" false)
    (assertIsValidVersion "0.10.0-dev" "^0.10.2" false)
    (assertIsValidVersion "0.10.0-dev" "0.10.x" true)
    (assertIsValidVersion "0.10.0-dev" "^0.10.0" true)
    (assertIsValidVersion "0.10.0-dev" "*" true)
    (assertIsValidVersion "0.10.0-dev" ">=0.0.1" true)
    (assertIsValidVersion "0.10.0-dev" ">=0.0.10" true)
    (assertIsValidVersion "0.10.0-dev" ">=0.10.0" true)
    (assertIsValidVersion "0.10.0-dev" ">=0.10.1" false)
    (assertIsValidVersion "0.10.0-dev" ">=1.0.0" false)

    (assertIsValidVersion "0.10.0" "x.x.x" true)
    (assertIsValidVersion "0.10.0" "0.x.x" true)
    (assertIsValidVersion "0.10.0" "0.10.0" true)
    (assertIsValidVersion "0.10.0" "0.10.2" false)
    (assertIsValidVersion "0.10.0" "^0.10.2" false)
    (assertIsValidVersion "0.10.0" "0.10.x" true)
    (assertIsValidVersion "0.10.0" "^0.10.0" true)
    (assertIsValidVersion "0.10.0" "*" true)

    (assertIsValidVersion "0.10.1" "x.x.x" true)
    (assertIsValidVersion "0.10.1" "0.x.x" true)
    (assertIsValidVersion "0.10.1" "0.10.0" false)
    (assertIsValidVersion "0.10.1" "0.10.2" false)
    (assertIsValidVersion "0.10.1" "^0.10.2" false)
    (assertIsValidVersion "0.10.1" "0.10.x" true)
    (assertIsValidVersion "0.10.1" "^0.10.0" true)
    (assertIsValidVersion "0.10.1" "*" true)

    (assertIsValidVersion "0.10.100" "x.x.x" true)
    (assertIsValidVersion "0.10.100" "0.x.x" true)
    (assertIsValidVersion "0.10.100" "0.10.0" false)
    (assertIsValidVersion "0.10.100" "0.10.2" false)
    (assertIsValidVersion "0.10.100" "^0.10.2" true)
    (assertIsValidVersion "0.10.100" "0.10.x" true)
    (assertIsValidVersion "0.10.100" "^0.10.0" true)
    (assertIsValidVersion "0.10.100" "*" true)

    (assertIsValidVersion "0.11.0" "x.x.x" true)
    (assertIsValidVersion "0.11.0" "0.x.x" true)
    (assertIsValidVersion "0.11.0" "0.10.0" false)
    (assertIsValidVersion "0.11.0" "0.10.2" false)
    (assertIsValidVersion "0.11.0" "^0.10.2" false)
    (assertIsValidVersion "0.11.0" "0.10.x" false)
    (assertIsValidVersion "0.11.0" "^0.10.0" false)
    (assertIsValidVersion "0.11.0" "*" true)

    # Anything < 1.0.0 is compatible

    (assertIsValidVersion "1.0.0" "x.x.x" true)
    (assertIsValidVersion "1.0.0" "0.x.x" true)
    (assertIsValidVersion "1.0.0" "0.10.0" false)
    (assertIsValidVersion "1.0.0" "0.10.2" false)
    (assertIsValidVersion "1.0.0" "^0.10.2" true)
    (assertIsValidVersion "1.0.0" "0.10.x" true)
    (assertIsValidVersion "1.0.0" "^0.10.0" true)
    (assertIsValidVersion "1.0.0" "1.0.0" true)
    (assertIsValidVersion "1.0.0" "^1.0.0" true)
    (assertIsValidVersion "1.0.0" "^2.0.0" false)
    (assertIsValidVersion "1.0.0" "*" true)
    (assertIsValidVersion "1.0.0" ">=0.0.1" true)
    (assertIsValidVersion "1.0.0" ">=0.0.10" true)
    (assertIsValidVersion "1.0.0" ">=0.10.0" true)
    (assertIsValidVersion "1.0.0" ">=0.10.1" true)
    (assertIsValidVersion "1.0.0" ">=1.0.0" true)
    (assertIsValidVersion "1.0.0" ">=1.1.0" false)
    (assertIsValidVersion "1.0.0" ">=1.0.1" false)
    (assertIsValidVersion "1.0.0" ">=2.0.0" false)

    (assertIsValidVersion "1.0.100" "x.x.x" true)
    (assertIsValidVersion "1.0.100" "0.x.x" true)
    (assertIsValidVersion "1.0.100" "0.10.0" false)
    (assertIsValidVersion "1.0.100" "0.10.2" false)
    (assertIsValidVersion "1.0.100" "^0.10.2" true)
    (assertIsValidVersion "1.0.100" "0.10.x" true)
    (assertIsValidVersion "1.0.100" "^0.10.0" true)
    (assertIsValidVersion "1.0.100" "1.0.0" false)
    (assertIsValidVersion "1.0.100" "^1.0.0" true)
    (assertIsValidVersion "1.0.100" "^1.0.1" true)
    (assertIsValidVersion "1.0.100" "^2.0.0" false)
    (assertIsValidVersion "1.0.100" "*" true)

    (assertIsValidVersion "1.100.0" "x.x.x" true)
    (assertIsValidVersion "1.100.0" "0.x.x" true)
    (assertIsValidVersion "1.100.0" "0.10.0" false)
    (assertIsValidVersion "1.100.0" "0.10.2" false)
    (assertIsValidVersion "1.100.0" "^0.10.2" true)
    (assertIsValidVersion "1.100.0" "0.10.x" true)
    (assertIsValidVersion "1.100.0" "^0.10.0" true)
    (assertIsValidVersion "1.100.0" "1.0.0" false)
    (assertIsValidVersion "1.100.0" "^1.0.0" true)
    (assertIsValidVersion "1.100.0" "^1.1.0" true)
    (assertIsValidVersion "1.100.0" "^1.100.0" true)
    (assertIsValidVersion "1.100.0" "^2.0.0" false)
    (assertIsValidVersion "1.100.0" "*" true)
    (assertIsValidVersion "1.100.0" ">=1.99.0" true)
    (assertIsValidVersion "1.100.0" ">=1.100.0" true)
    (assertIsValidVersion "1.100.0" ">=1.101.0" false)

    (assertIsValidVersion "2.0.0" "x.x.x" true)
    (assertIsValidVersion "2.0.0" "0.x.x" false)
    (assertIsValidVersion "2.0.0" "0.10.0" false)
    (assertIsValidVersion "2.0.0" "0.10.2" false)
    (assertIsValidVersion "2.0.0" "^0.10.2" false)
    (assertIsValidVersion "2.0.0" "0.10.x" false)
    (assertIsValidVersion "2.0.0" "^0.10.0" false)
    (assertIsValidVersion "2.0.0" "1.0.0" false)
    (assertIsValidVersion "2.0.0" "^1.0.0" false)
    (assertIsValidVersion "2.0.0" "^1.1.0" false)
    (assertIsValidVersion "2.0.0" "^1.100.0" false)
    (assertIsValidVersion "2.0.0" "^2.0.0" true)
    (assertIsValidVersion "2.0.0" "*" true)
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

  resultExact = reports: {
    success = builtins.foldl' (x: y: x + y) 0 (
      map (report: builtins.length (builtins.filter (v: v.value == v.expected) report)) reports
    );
    failed = builtins.foldl' (x: y: x + y) 0 (
      map (report: builtins.length (builtins.filter (v: v.value != v.expected) report)) reports
    );
    report = builtins.concatMap reportExact reports;
  };

  testReport =
    let
      result = resultExact [
        isVersionValidStrReport
        parseVersionReport
        normalizeVersionReport
        isValidVersionReport
      ];
    in
    {
      success = result.success;
      failed = result.failed;
      report = result.report;
    };
in
{
  inherit
    testReport
    utils
    lib
    pkgs
    ;
  isVersionValidStrRes = isVersionValidStrReport;
}
