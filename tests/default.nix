{ lib }:
{
  matchesVscodeVersion = import ./matchesVscodeVersion.nix {
    inherit lib;
  };
  upstreamVersionTests = import ./upstreamVersionTests.nix {
    inherit lib;
  };
}
