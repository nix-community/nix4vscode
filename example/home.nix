{
  pkgs,
  ...
}:

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
          extensions = pkgs.nix4vscode.forVscodeVersion "1.100.2" [
            "vadimcn.vscode-lldb"
            "ms-vscode.cpptools"
            "jnoortheen.nix-ide"
            # theme
            "ms-ceintl.vscode-language-pack-zh-hans"
            "zhuangtongfa.material-theme"
          ];
        };
      };
    };
  };

}
