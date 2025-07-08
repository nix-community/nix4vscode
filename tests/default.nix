{ lib }:
{
  matchesVscodeVersion = import ./matchesVscodeVersion.nix {
    inherit lib;
  };
}
