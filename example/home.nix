{
  pkgs,
  ...
}:

let
  inherit (pkgs.nix4vscode)
    forVscode
    forVscodeVersion
    forVscodePrerelease
    forVscodeVersionPrerelease

    forOpenVsx
    forOpenVsxVersion
    forOpenVsxPrerelease
    forOpenVsxVersionPrerelease

    forVscodeExt
    forVscodeExtVersion
    forVscodeExtPrerelease
    forVscodeExtVersionPrerelease

    forOpenVsxExt
    forOpenVsxExtVersion
    forOpenVsxExtPrerelease
    forOpenVsxExtVersionPrerelease
    ;

  myDecorators = {
    "ms-vscode.cpptools" = {
      postPatch = ''
        echo "Custom decorator applied"
      '';
    };
  };
in

{
  home.username = "root";
  home.homeDirectory = "/root";
  home.stateVersion = "24.05";

  programs = {
    vscode = {
      enable = true;
      profiles = {
        default = {
          enableExtensionUpdateCheck = false;
          enableUpdateCheck = false;
          extensions =
            forVscode [
              "zhuangtongfa.material-theme"
            ]
            # issue #338
            ++ forVscodeVersion "1.104.0" [
              "github.copilot-chat.0.31.0"
            ]
            ++ forVscodeVersion "1.100.2" [
              "vadimcn.vscode-lldb"
            ]
            ++ forVscodePrerelease [
              "ms-toolsai.vscode-jupyter-slideshow"
              "ms-dotnettools.csdevkit"
            ]
            ++ forVscodeVersionPrerelease "1.100.2" [
              "ms-toolsai.vscode-jupyter-cell-tags"
              "ms-dotnettools.csharp"
            ]
            ++ forVscodeExt myDecorators [
              "ms-vscode.cpptools"
            ]
            ++ forVscodeExtPrerelease myDecorators [
              "matepek.vscode-catch2-test-adapter"
            ]
            ++ forVscodeExtVersionPrerelease myDecorators "1.100.2" [
              "ms-azuretools.vscode-containers"
            ]
            ++ forOpenVsx [
              "jnoortheen.nix-ide"
            ]
            ++ forOpenVsxVersion "1.100.2" [
              "ms-ceintl.vscode-language-pack-zh-hans"
            ]
            ++ forOpenVsxPrerelease [
              "Dart-Code.flutter"
            ]
            ++ forOpenVsxVersionPrerelease "1.100.2" [
              "Continue.continue"
            ]
            ++ forVscodeExtVersion myDecorators "1.100.2" [
              "ms-python.python"
            ]
            ++ forOpenVsxExt myDecorators [
              "redhat.java"
            ]
            ++ forOpenVsxExtVersion myDecorators "1.100.2" [
              "charliermarsh.ruff"
            ]
            ++ forOpenVsxExtPrerelease myDecorators [
              "svelte.svelte-vscode"
            ]
            ++ forOpenVsxExtVersionPrerelease myDecorators "1.100.2" [
              "posit.shiny"
            ];
        };
      };
    };
  };

}
