/**
  Indicate if the given version selector matches the vscode version.

  - `^M.m.p` means `>=M.m.p` if `vscodeVersion>0`, but `>=M.m.p && <M.(m+1).0` if `vscodeVersion=0`.
  - `~M.m.p` is undocumented. We assume it means `>=M.m.p && <M.(m+1).0`
  - When one of `M`, `m`, or `p` is `x`, it means "anything".
  - When `M` is `0`, it matches with `1` too.
  - `*` means "anything".

  DOCS https://code.visualstudio.com/api/working-with-extensions/publishing-extension#visual-studio-code-compatibility

  # Type

  ```
  matchesVscodeVersion :: string -> boolean
  ```

  # Example

  For examples and expected results, see ../tests/matchesVscodeVersion.nix
  ```
*/
lib: vscodeVersion: versionSelector:
let
  operator =
    if lib.hasPrefix ">=" versionSelector then
      ">="
    else if lib.hasPrefix ">" versionSelector then
      ">"
    else if lib.hasPrefix "^" versionSelector then
      "^"
    else if lib.hasPrefix "~" versionSelector then
      "~"
    else if
      builtins.elem versionSelector [
        "*"
        "x.x.x"
      ]
    then
      "*"
    else
      "=";
  # Current vscode version
  vscodeMajor = lib.versions.major vscodeVersion;
  vscodeMinor = lib.versions.minor vscodeVersion;
  vscodePatch = lib.versions.patch vscodeVersion;

  # Wanted version
  version = lib.removePrefix operator versionSelector;
  major = lib.versions.major version;
  minor = lib.versions.minor version;
  patch = lib.versions.patch version;
  labels = lib.removePrefix "${major}.${minor}.${patch}" version;

  # Next version
  nextMajor = builtins.toString (lib.toInt major + (if major == "0" then 2 else 1));
  nextMinor = builtins.toString (lib.toInt minor + 1);

  # Quirks
  xMajor = major == "x" || (major == "0" && vscodeMajor == "1" && (xMinor || xPatch));
  xMinor = minor == "x";
  xPatch = patch == "x";
  xChangedPrefix = lib.concatStrings [
    (lib.concatStringsSep "." [
      (if xMajor then vscodeMajor else major)
      (if xMajor || xMinor then vscodeMinor else minor)
      (if xMajor || xMinor || xPatch then vscodePatch else patch)
    ])
    labels
  ];

  results = {
    ">=" = lib.versionAtLeast vscodeVersion version;
    ">" = !results."=" && results.">=";
    "^" =
      results.">="
      && lib.versionOlder vscodeVersion (
        if major == "0" && vscodeMajor == "0" then "0.${nextMinor}" else nextMajor
      );
    "~" = results.">=" && lib.versionOlder vscodeVersion "${major}.${nextMinor}";
    "=" =
      xChangedPrefix == vscodeVersion || xChangedPrefix == "${vscodeMajor}.${vscodeMinor}.${vscodePatch}";
    "*" = true;
  };
in
results.${operator}
