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
            ++ forVscodeVersion "1.100.2" [
              "vadimcn.vscode-lldb"
              "jnoortheen.nix-ide"
              # theme
              "ms-ceintl.vscode-language-pack-zh-hans"
            ]
            ++ forVscodePrerelease [
              "ms-toolsai.vscode-jupyter-slideshow"
            ]
            ++ forVscodeVersionPrerelease "1.100.2" [
              "ms-toolsai.vscode-jupyter-cell-tags"
            ]
            ++ forVscodeExt myDecorators [
              "ms-vscode.cpptools"
            ];
          # ++ forOpenVsx [
          #   "redhat.java"
          # ];
          # ++ forOpenVsxVersion "1.100.2" [
          # "llvm-vs-code-extensions.vscode-clangd"
          # ]
          # ++ forOpenVsxPrerelease [
          #   "Dart-Code.flutter"
          # ]
          # ++ forOpenVsxVersionPrerelease "1.100.2" [
          #   "Continue.continue"
          # ];
          # ++ forVscodeExtVersion myDecorators "1.100.2" [
          #   "ms-python.python"
          # ]
          # ++ forOpenVsxExt myDecorators [
          #   "redhat.java"
          # ];
        };
      };
    };
  };

}
