{ ... }:

{

  postPatch = ''
    # Patching binaries
    make_executable() {
      for file in "''$@"; do
        if [[ -e "''$file" ]]; then
          chmod +x "''$file"
        fi
      done
    }

    make_executable server/bin/lua-language-server
    sed -i "s/await fs.promises.chmod(command, \"777\");//g" client/out/src/languageserver.js
  '';
}
