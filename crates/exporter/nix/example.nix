# Example: How to use vscodeExtensionsForEngine and vscodeExtensionsForEnginePlatform
# Can be run with the following command:
# nix eval --file ./example.nix

let
  # Import flake
  flake = builtins.getFlake (toString ./.);

  # Format output results
  formatExtension = ext: {
    name = "${ext.publisher}.${ext.name}";
    version = ext.version;
    engine = ext.engine;
    platform = ext.platform;
  };

  # Create a function to get extensions based on engine version
  getExtensionsForEngine =
    engineVersion:
    let
      # Get the list of extensions matching the engine version
      matchingExtensions = flake.lib.vscodeExtensionsForEngine engineVersion;
      # Convert results to a more readable format
      result = map formatExtension matchingExtensions;
    in
    {
      engineVersion = engineVersion;
      matchingExtensionsCount = builtins.length matchingExtensions;
      extensions = result;
    };

  # Create a function to get extensions based on engine version and platform
  getExtensionsForEnginePlatform =
    engineVersion: platform:
    let
      # Get the list of extensions matching the engine version and platform
      matchingExtensions = flake.lib.vscodeExtensionsForEnginePlatform engineVersion platform;
      # Convert results to a more readable format
      result = map formatExtension matchingExtensions;
    in
    {
      engineVersion = engineVersion;
      platform = platform;
      matchingExtensionsCount = builtins.length matchingExtensions;
      extensions = result;
    };

  # Predefine some common VSCode engine versions
  commonEngineVersions = [
    "1.85.0"
    "1.86.0"
    "1.87.0"
    "1.88.0"
  ];

  # Predefine some common platforms
  commonPlatforms = [
    "web"
    "linux-x64"
    "darwin-x64"
    "darwin-arm64"
  ];

  # Create results for each common version
  versionResults = builtins.listToAttrs (
    map (version: {
      name = version;
      value = getExtensionsForEngine version;
    }) commonEngineVersions
  );

  # Create results for specific version and platform combinations
  platformResults = builtins.listToAttrs (
    map (platform: {
      name = "1.85.0-${platform}";
      value = getExtensionsForEnginePlatform "1.85.0" platform;
    }) commonPlatforms
  );
in
# Return an object that can be indexed by version number
versionResults
// platformResults
// {
  # Add a function to get any arbitrary version
  __functor =
    self: arg1: arg2:
    if arg2 == null then getExtensionsForEngine arg1 else getExtensionsForEnginePlatform arg1 arg2;
}
