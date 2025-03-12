{
  pkgs ? import <nixpkgs> { },
}:

let
  lib = pkgs.lib;
  versionUtils = import ./version-utils.nix { inherit lib; };

  # Define test cases
  testCases = [
    # Basic version matching
    {
      codeVersion = "1.8.1";
      requestedVersion = "1.8.1";
      expected = true;
      description = "Exact version match";
    }
    {
      codeVersion = "1.8.1";
      requestedVersion = "1.8.0";
      expected = false;
      description = "Different patch version";
    }
    {
      codeVersion = "1.8.1";
      requestedVersion = "1.7.1";
      expected = false;
      description = "Different minor version";
    }
    {
      codeVersion = "1.8.1";
      requestedVersion = "2.8.1";
      expected = false;
      description = "Different major version";
    }

    # Caret ranges
    {
      codeVersion = "1.8.1";
      requestedVersion = "^1.2.1";
      expected = true;
      description = "Caret range - compatible version";
    }
    {
      codeVersion = "1.8.1";
      requestedVersion = "^1.9.0";
      expected = false;
      description = "Caret range - incompatible version";
    }
    {
      codeVersion = "1.8.1";
      requestedVersion = "^2.0.0";
      expected = false;
      description = "Caret range - different major version";
    }
    {
      codeVersion = "0.8.1";
      requestedVersion = "^0.8.0";
      expected = true;
      description = "Caret range - 0.x.x compatible patch";
    }
    {
      codeVersion = "0.9.1";
      requestedVersion = "^0.8.0";
      expected = false;
      description = "Caret range - 0.x.x incompatible minor";
    }

    # Wildcard versions
    {
      codeVersion = "1.8.1";
      requestedVersion = "*";
      expected = true;
      description = "Wildcard version";
    }
    {
      codeVersion = "1.8.1";
      requestedVersion = "1.8.x";
      expected = true;
      description = "Patch wildcard - matching";
    }
    {
      codeVersion = "1.9.1";
      requestedVersion = "1.8.x";
      expected = false;
      description = "Patch wildcard - non-matching";
    }
    {
      codeVersion = "1.8.1";
      requestedVersion = "1.x.x";
      expected = true;
      description = "Minor and patch wildcard - matching";
    }
    {
      codeVersion = "2.8.1";
      requestedVersion = "1.x.x";
      expected = false;
      description = "Minor and patch wildcard - non-matching";
    }

    # Greater than or equal
    {
      codeVersion = "1.8.1";
      requestedVersion = ">=1.8.0";
      expected = true;
      description = "Greater than or equal - higher patch";
    }
    {
      codeVersion = "1.8.1";
      requestedVersion = ">=1.8.2";
      expected = false;
      description = "Greater than or equal - lower patch";
    }
    {
      codeVersion = "1.9.0";
      requestedVersion = ">=1.8.0";
      expected = true;
      description = "Greater than or equal - higher minor";
    }
    {
      codeVersion = "1.7.0";
      requestedVersion = ">=1.8.0";
      expected = false;
      description = "Greater than or equal - lower minor";
    }
    {
      codeVersion = "2.0.0";
      requestedVersion = ">=1.8.0";
      expected = true;
      description = "Greater than or equal - higher major";
    }
    {
      codeVersion = "1.0.0";
      requestedVersion = ">=2.0.0";
      expected = false;
      description = "Greater than or equal - lower major";
    }

    # Special cases from Rust tests
    {
      codeVersion = "1.8.1";
      requestedVersion = "^0.1.1";
      expected = true;
      description = "Special case from Rust tests";
    }
    {
      codeVersion = "1.8.1";
      requestedVersion = "1.81.1-inside";
      expected = false;
      description = "Special case from Rust tests";
    }
    {
      codeVersion = "1.8.1";
      requestedVersion = "1.0.1-inside";
      expected = false;
      description = "Special case from Rust tests";
    }
    {
      codeVersion = "1.8.1";
      requestedVersion = "^1.81.1-inside";
      expected = false;
      description = "Special case from Rust tests";
    }
    {
      codeVersion = "1.8.1";
      requestedVersion = "^1.0.1-inside";
      expected = true;
      description = "Special case from Rust tests";
    }
    {
      codeVersion = "1.8.1";
      requestedVersion = "1.12.1";
      expected = false;
      description = "Special case from Rust tests";
    }
    {
      codeVersion = "1.8.1";
      requestedVersion = "^1.12.1";
      expected = false;
      description = "Special case from Rust tests";
    }
    {
      codeVersion = "1.8.1";
      requestedVersion = "0.10.x";
      expected = true;
      description = "Special case from Rust tests";
    }
    {
      codeVersion = "1.8.1";
      requestedVersion = "^0.10.x";
      expected = true;
      description = "Special case from Rust tests";
    }

    # Edge cases
    {
      codeVersion = "1.0.0";
      requestedVersion = "^0.9.0";
      expected = true;
      description = "1.0.0 is compatible with ^0.9.0";
    }
    {
      codeVersion = "2.0.0";
      requestedVersion = "^1.9.0";
      expected = false;
      description = "2.0.0 is not compatible with ^1.9.0";
    }
    {
      codeVersion = "0.2.0";
      requestedVersion = "^0.1.0";
      expected = false;
      description = "0.2.0 is not compatible with ^0.1.0";
    }
    {
      codeVersion = "0.1.2";
      requestedVersion = "^0.1.0";
      expected = true;
      description = "0.1.2 is compatible with ^0.1.0";
    }

    # Invalid versions
    {
      codeVersion = "1.8.1";
      requestedVersion = "invalid";
      expected = false;
      description = "Invalid requested version";
    }
    {
      codeVersion = "invalid";
      requestedVersion = "1.8.1";
      expected = false;
      description = "Invalid code version";
    }
    {
      codeVersion = "invalid";
      requestedVersion = "invalid";
      expected = false;
      description = "Both versions invalid";
    }
  ];

  # Run tests
  runTest =
    testCase:
    let
      result = versionUtils.isVersionValid testCase.codeVersion testCase.requestedVersion;
      success = result == testCase.expected;
      status = if success then "PASS" else "FAIL";
    in
    {
      inherit success;
      output = "${status}: ${testCase.description} - Code version: ${testCase.codeVersion}, Requested version: ${testCase.requestedVersion}, Expected: ${toString testCase.expected}, Got: ${toString result}";
    };

  testResults = map runTest testCases;
  passedTests = lib.filter (result: result.success) testResults;
  failedTests = lib.filter (result: !result.success) testResults;

  # Summary
  summary = ''
    Version Utils Test Results
    ========================
    Total tests: ${toString (builtins.length testCases)}
    Passed: ${toString (builtins.length passedTests)}
    Failed: ${toString (builtins.length failedTests)}

    ${if (builtins.length failedTests) > 0 then "Failed Tests:" else ""}
    ${lib.concatMapStrings (result: "- ${result.output}\n") failedTests}

    ${if (builtins.length passedTests) > 0 then "Passed Tests:" else ""}
    ${lib.concatMapStrings (result: "- ${result.output}\n") passedTests}
  '';

  # Create a derivation that runs the tests
  testScript = pkgs.writeTextFile {
    name = "version-utils-test-results";
    text = summary;
    executable = true;
  };

  # Create a derivation that fails if any tests fail
  testDerivation =
    pkgs.runCommand "version-utils-test"
      {
        buildInputs = [ ];
        passthru = {
          inherit
            testResults
            passedTests
            failedTests
            summary
            ;
        };
      }
      ''
        echo "${summary}" > $out
        ${if (builtins.length failedTests) > 0 then "exit 1" else ""}
      '';
in
{
  # Return the test derivation
  test = testDerivation;

  # Return a function to run the tests and print results
  runTests = pkgs.writeShellScriptBin "run-version-utils-tests" ''
    echo "${summary}"
    ${if (builtins.length failedTests) > 0 then "exit 1" else "exit 0"}
  '';
}
