{
  pkgs,
  dataPath ? if isOpenVsx then ../data/extensions_openvsx.json else ../data/extensions.json,
  version ? pkgs.vscode.version,
  extensions,
  pickPreRelease ? false,
  decorators ? null,
  isOpenVsx ? false,
}:
let
  inherit (pkgs) lib system;
  allExtensions = lib.importJSON dataPath;
  platformMap = {
    x86_64-linux = "linux-x64";
    i686-linux = "linux-x64";
    aarch64-linux = "linux-arm64";
    armv7l-linux = "linux-armhf";
    x86_64-darwin = "darwin-x64";
    aarch64-darwin = "darwin-arm64";
  };
  platform = platformMap.${system} or null;

  # Get an attrset of extensions and their wanted versions
  wantedExts = builtins.listToAttrs (lib.map splitExt extensions);

  /**
    Split an extension name into its parts and return a name-value pair.
    The name is the first two parts of the extension name, and the value is the version.

    # Type

    ```
    splitExt :: string -> { name :: string; value :: string; }
    ```

    # Example

    ```nix
    publisher.name.1.2.3" -> { name = "publisher.name"; value = "1.2.3" }
    ```
  */
  splitExt =
    name:
    let
      parts = lib.splitString "." name;
      names = lib.take 2 parts;
      fullName = lib.concatStringsSep "." names;
      versions = lib.drop 2 parts;
      fullVersion = lib.concatStringsSep "." versions;
    in
    lib.nameValuePair (lib.toLower fullName) fullVersion;

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

    In the following examples, the contextual `vscodeVersion` is `"1.50.3"`:

    ```nix
    matchesVscodeVersion ">=1.50.1" -> true
    matchesVscodeVersion ">1.50.1" -> true
    matchesVscodeVersion "^1.50.1" -> true
    matchesVscodeVersion "1.50.1" -> false
    matchesVscodeVersion "~1.50.1" -> true

    matchesVscodeVersion ">=1.49.1" -> true
    mmatchesVscodeVersion ">1.49.1" -> true
    matchesVscodeVersion "^1.49.1" -> true
    matchesVscodeVersion "1.49.1" -> false
    matchesVscodeVersion "~1.49.1" -> false

    matchesVscodeVersion ">=0.99.1" -> true
    matchesVscodeVersion "^0.99.1" -> false
    matchesVscodeVersion "0.99.1" -> false
    matchesVscodeVersion "~0.99.1" -> false
    ```
  */
  matchesVscodeVersion =
    versionSelector:
    let
      operator =
        if lib.hasPrefix ">=" versionSelector then
          ">="
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
        ">=" = lib.versionAtLeast version version;
        ">" = !results."=" && results.">=";
        "^" = results.">=";
        "~" = results.">=" && lib.versionOlder "${major}.${nextMinor}" version;
        "=" = version == version;
      };
    in
    results.${operator};

  /**
    Get all versions of an extension from the allExtensions attribute set.
    Returns a list of versions for the given extension name.

    # Type

    ```
    getBestVersionSpec :: string -> [{v :: string; e :: string; h :: string; p? :: string; r? :: bool; }] -> {v :: string; e :: string; h :: string; p? :: string; r? :: bool; }
    ```

    Meaning of the keys:
    - `v`: The version of the extension.
    - `e`: The version selector for the extension, which should match the vscode version.
    - `h`: The hash of the extension.
    - `p`: The platform for which the extension is built (optional).
    - `r`: Whether the extension is a pre-release version (optional, defaults to false).

    # Example

    ```nix
    getBestVersionSpec "2.0.0" [
      { v = "1.0.0"; e = ">=0.9.0-pre.1"; h = "hash1"; p = "linux-x64"; r = false; }
      { v = "2.0.0"; e = ">=0.9.0-pre.1"; h = "hash2"; p = "linux-x64"; r = false; }
      { v = "2.1.0"; e = "^1.0.0"; h = "hash3"; p = "linux-x64"; r = true; }
    ]
    -> { v = "2.0.0"; e = "^1.0.0"; h = "hash2"; p = "linux-x64"; r = false; }
    ```
  */
  getBestVersionSpec =
    name: wantedVersion:
    let
      fullSpec = allExtensions.${name} or (throw "Extension ${name} not found in ${dataPath}");
      filteredSpec = lib.filter (
        spec:
        (spec.p or platform) == platform
        && (pickPreRelease || !(spec.r or false))
        && matchesVscodeVersion spec.e
      ) fullSpec;
      latestVersion = lib.foldl' (acc: spec: if lib.versionAtLeast spec.v acc.v then spec else acc) {
        v = "0.0.0";
      } filteredSpec;
      matchingVersions = builtins.map (spec: spec.v) filteredSpec;
      reallyWantedVersion =
        if wantedVersion == "" || wantedVersion == "*" then
          latestVersion.v
        else if lib.elem wantedVersion matchingVersions then
          wantedVersion
        else
          throw "Extension ${wantedVersion} not found in ${lib.concatStringsSep ", " matchingVersions}";
    in
    lib.findFirst (
      v: v.v == reallyWantedVersion
    ) (throw "Could not find version '${reallyWantedVersion}' for extension ${name}") filteredSpec;

  filteredExtensions = lib.mapAttrs getBestVersionSpec wantedExts;

  vscode-marketplace = import ./extensionsFromInfo.nix {
    inherit
      decorators
      isOpenVsx
      lib
      pkgs
      system
      ;
    extensions = filteredExtensions;
  };
  listDifference = a: b: builtins.filter (x: !(builtins.elem x b)) a;
  names = builtins.map (
    item:
    let
      parts = lib.strings.splitString "." item;
    in
    if builtins.length parts < 3 then
      (lib.strings.toLower item)
    else
      (lib.strings.toLower (builtins.concatStringsSep "." (lib.lists.take 2 parts)))
  ) extensions;
  attrs = builtins.attrNames vscode-marketplace;
  diff = listDifference names (builtins.map (x: lib.strings.toLower x) attrs);

  validateAttribute =
    if builtins.length diff == 0 then
      vscode-marketplace
    else
      throw ''
        The following extensions were not found: ${builtins.concatStringsSep "," diff}
        1) Is there a spelling error? (Case insensitive)
        2) Is there a version of the specified extension suitable for vscode `${version}`
        3) If the specified extension has no stable version? If not, you may need forVscodePrerelease
      '';
in
builtins.attrValues validateAttribute
