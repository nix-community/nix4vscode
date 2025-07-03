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
    ;
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
              "ms-vscode.cpptools"
              "jnoortheen.nix-ide"
              # theme
              "ms-ceintl.vscode-language-pack-zh-hans"
            ]
            ++ forVscodePrerelease [
              "ms-toolsai.vscode-jupyter-slideshow"
            ]
            ++ forVscodeVersionPrerelease "1.100.2" [
              "ms-toolsai.vscode-jupyter-cell-tags"
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
        };
      };
    };
  };

}
