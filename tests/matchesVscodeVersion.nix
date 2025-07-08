{ lib }:
let
  matchesVscodeVersion = import ../nix/matchesVscodeVersion.nix lib;
  matches_1_50_3 = matchesVscodeVersion "1.50.3";
  works = selector: {
    expr = matches_1_50_3 selector;
    expected = true;
  };
  fails = selector: {
    expr = matches_1_50_3 selector;
    expected = false;
  };
in
{
  testGT_0_999_0 = works ">0.999.0";
  testGT_1_49_999 = works ">1.49.999";
  testGT_1_50_2 = works ">1.50.2";
  testGT_1_50_3 = fails ">1.50.3";
  testGT_1_51_0 = fails ">1.51.0";
  testGT_2_0_0 = fails ">2.0.0";

  testGE_0_999_0 = works ">=0.999.0";
  testGE_1_49_999 = works ">=1.49.999";
  testGE_1_50_2 = works ">=1.50.2";
  testGE_1_50_3 = works ">=1.50.3";
  testGE_1_51_0 = fails ">=1.51.0";
  testGE_2_0_0 = fails ">=2.0.0";

  testCaret_0_999_0 = works "^0.999.0";
  testCaret_1_49_999 = works "^1.49.999";
  testCaret_1_50_2 = works "^1.50.2";
  testCaret_1_50_3 = works "^1.50.3";
  testCaret_1_51_0 = fails "^1.51.0";
  testCaret_2_0_0 = fails "^2.0.0";

  testTilde_0_999_0 = fails "~0.999.0";
  testTilde_1_49_999 = fails "~1.49.999";
  testTilde_1_50_2 = works "~1.50.2";
  testTilde_1_50_3 = works "~1.50.3";
  testTilde_1_51_0 = fails "~1.51.0";
  testTilde_2_0_0 = fails "~2.0.0";

  testEq_0_999_0 = fails "0.999.0";
  testEq_1_49_999 = fails "1.49.999";
  testEq_1_50_2 = fails "1.50.2";
  testEq_1_50_3 = works "1.50.3";
  testEq_1_51_0 = fails "1.51.0";
  testEq_2_0_0 = fails "2.0.0";
}
