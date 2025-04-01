# https://github.com/NixOS/nixpkgs/raw/refs/heads/master/pkgs/applications/editors/vscode/extensions/ms-vscode.cpptools/default.nix
{
  pkgs ? import <nixpkgs> { },
  lib ? pkgs.lib,
  system ? builtins.currentSystem,
  gdbUseFixed ? true,
  gdb ? pkgs.gdb, # The gdb default setting will be fixed to specified. Use version from `PATH` otherwise.
  clang-tools ? pkgs.clang-tools,
}:

{

  postPatch = ''
    # 1. Add activation events so that the extension is functional. This listing is empty when unpacking the extension but is filled at runtime.
    # 2. Patch `package.json` so that nix's *gdb* is used as default value for `miDebuggerPath`.
    # Prevent download/install of extensions
    touch "./install.lock"

    # Clang-format from nix package.
    rm -rf ./LLVM
    mkdir "./LLVM/"
    ln -s ${clang-tools}/bin ./LLVM/bin

    # Patching binaries
    make_executable() {
      for file in "''$@"; do
        if [[ -e "''$file" ]]; then
          chmod +x "''$file"
        fi
      done
    }

    make_executable bin/cpptools bin/cpptools-srv debugAdapters/bin/OpenDebugAD7 bin/libc.so bin/cpptools-wordexp debugAdapters/lldb-mi/bin/lldb-mi debugAdapters/bin/createdump
  '';
}
