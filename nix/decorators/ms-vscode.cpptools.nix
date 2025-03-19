# https://github.com/NixOS/nixpkgs/raw/refs/heads/master/pkgs/applications/editors/vscode/extensions/ms-vscode.cpptools/default.nix
{
  pkgs ? import <nixpkgs> { },
  lib ? pkgs.lib,
  system ? builtins.currentSystem,
  gdbUseFixed ? true,
  gdb ? pkgs.gdb, # The gdb default setting will be fixed to specified. Use version from `PATH` otherwise.
  clang-tools ? pkgs.clang-tools,
}:

let
  isx86Linux = system == "x86_64-linux";
  isLinux = system == "x86_64-linux" || system == "aarch64-linux";
  gdbUseFixedOverlay = gdbUseFixed && isLinux;
in
{
  nativeBuildInputs =
    with pkgs;
    [
      makeWrapper
    ]
    ++ lib.lists.optional isLinux [
      autoPatchelfHook
    ];

  buildInputs =
    with pkgs;
    [
      jq
      libkrb5
      zlib
      (lib.getLib stdenv.cc.cc)
    ]
    ++ lib.lists.optional isLinux [
      lttng-ust
    ];

  dontAutoPatchelf = isx86Linux;

  postPatch = ''
    mv ./package.json ./package_orig.json

    # 1. Add activation events so that the extension is functional. This listing is empty when unpacking the extension but is filled at runtime.
    # 2. Patch `package.json` so that nix's *gdb* is used as default value for `miDebuggerPath`.
    # Prevent download/install of extensions
    touch "./install.lock"

    # Clang-format from nix package.
    rm -rf ./LLVM
    mkdir "./LLVM/"
    find "${clang-tools}" -mindepth 1 -maxdepth 1 | xargs ln -s -t "./LLVM"

    # Patching binaries
    make_executable() {
      for file in "''$@"; do
        if [[ -e "''$file" ]]; then
          chmod +x "''$file"
        fi
      done
    }

    make_executable bin/cpptools bin/cpptools-srv debugAdapters/bin/OpenDebugAD7 bin/libc.so bin/cpptools-wordexp
    #
    patch_elf() {
      for file in "''$@"; do
        if [[ -e "''$file" ]]; then
          patchelf --replace-needed "''$file"
        fi
      done
    }
    patch_elf liblttng-ust.so.0 liblttng-ust.so.1 ./debugAdapters/bin/libcoreclrtraceptprovider.so
  '';

  # On aarch64 the binaries are statically linked
  # but on x86 they are not.
  postFixup =
    lib.optionalString isx86Linux ''
      autoPatchelf $out/share/vscode/extensions/ms-vscode.cpptools/debugAdapters
      # cpptools* are distributed by the extension and need to be run through the distributed musl interpretter
      patchelf --set-interpreter $out/share/vscode/extensions/ms-vscode.cpptools/bin/libc.so $out/share/vscode/extensions/ms-vscode.cpptools/bin/cpptools
      patchelf --set-interpreter $out/share/vscode/extensions/ms-vscode.cpptools/bin/libc.so $out/share/vscode/extensions/ms-vscode.cpptools/bin/cpptools-srv
      patchelf --set-interpreter $out/share/vscode/extensions/ms-vscode.cpptools/bin/libc.so $out/share/vscode/extensions/ms-vscode.cpptools/bin/cpptools-wordexp
    ''
    + lib.optionalString gdbUseFixedOverlay ''
      wrapProgram $out/share/vscode/extensions/ms-vscode.cpptools/debugAdapters/bin/OpenDebugAD7 --prefix PATH : ${lib.makeBinPath [ gdb ]}
    '';
}
