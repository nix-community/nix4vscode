/**
  Indicate if the given version selector matches the vscode version.

  - `^M.m.p` means `>=M.m.p`
  - `~M.m.p` is undocumented. We assume it means `>=M.m.p && <M.(m+1).0`

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
    else
      "=";
  version = lib.removePrefix operator versionSelector;
  major = lib.versions.major version;
  minor = lib.versions.minor version;
  nextMinor = builtins.toString (lib.toInt minor + 1);
  results = {
    ">=" = lib.versionAtLeast vscodeVersion version;
    ">" = !results."=" && results.">=";
    "^" = results.">=";
    "~" = results.">=" && lib.versionOlder vscodeVersion "${major}.${nextMinor}";
    "=" = version == vscodeVersion;
  };
in
results.${operator}
