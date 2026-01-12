# https://github.com/NixOS/nixpkgs/blob/2a9f02d7fe1a70ffad3d93466f295c6c9a6b43f6/pkgs/applications/editors/vscode/extensions/vscode-utils.nix
{
  stdenv,
  lib,
  fetchurl,
  unzip,
  makeSetupHook,
  vscode-extension-update-script,
}:
let
  unpackVsixSetupHook = makeSetupHook {
    name = "unpack-vsix-setup-hook";
    substitutions = {
      unzip = "${unzip}/bin/unzip";
    };
  } ./unpack-vsix-setup-hook.sh;
  buildVscodeExtension = lib.extendMkDerivation {
    constructDrv = stdenv.mkDerivation;
    excludeDrvArgNames = [
      "vscodeExtUniqueId"
    ];
    extendDrvArgs =
      finalAttrs:
      {
        pname ? null, # Only optional for backward compatibility.
        # Same as "Unique Identifier" on the extension's web page.
        # For the moment, only serve as unique extension dir.
        vscodeExtPublisher,
        vscodeExtName,
        vscodeExtUniqueId,
        configurePhase ? ''
          runHook preConfigure
          runHook postConfigure
        '',
        buildPhase ? ''
          runHook preBuild
          runHook postBuild
        '',
        dontPatchELF ? true,
        dontStrip ? true,
        nativeBuildInputs ? [ ],
        passthru ? { },
        ...
      }@args:
      {
        pname = "vscode-extension-${pname}";

        passthru = {
          updateScript = vscode-extension-update-script { };
        }
        // passthru
        // {
          inherit vscodeExtPublisher vscodeExtName vscodeExtUniqueId;
        };

        inherit
          configurePhase
          buildPhase
          dontPatchELF
          dontStrip
          ;

        # Some .vsix files contain other directories (e.g., `package`) that we don't use.
        # If other directories are present but `sourceRoot` is unset, the unpacker phase fails.
        sourceRoot = args.sourceRoot or "extension";

        # This cannot be removed, it is used by some extensions.
        installPrefix = "share/vscode/extensions/${vscodeExtUniqueId}";

        nativeBuildInputs = [ unpackVsixSetupHook ] ++ nativeBuildInputs;

        installPhase =
          args.installPhase or ''
            runHook preInstall

            mkdir -p "$out/$installPrefix"
            find . -mindepth 1 -maxdepth 1 | xargs -d'\n' mv -t "$out/$installPrefix/"

            runHook postInstall
          '';
      };
  };

  fetchVsixFromVscodeMarketplace =
    mktplcExtRef: fetchurl (import ./mktplcExtRefToFetchArgs.nix mktplcExtRef);

  buildVscodeMarketplaceExtension = lib.extendMkDerivation {
    constructDrv = buildVscodeExtension;
    excludeDrvArgNames = [
      "mktplcRef"
      "vsix"
    ];
    extendDrvArgs =
      finalAttrs:
      {
        name ? "",
        src ? null,
        vsix ? null,
        mktplcRef,
        ...
      }:
      assert "" == name;
      assert null == src;
      {
        inherit (mktplcRef) version;
        pname = "${mktplcRef.publisher}-${mktplcRef.name}";
        src = if (vsix != null) then vsix else fetchVsixFromVscodeMarketplace mktplcRef;
        vscodeExtPublisher = mktplcRef.publisher;
        vscodeExtName = mktplcRef.name;
        vscodeExtUniqueId = "${mktplcRef.publisher}.${mktplcRef.name}";
      };
  };

in
{
  inherit
    buildVscodeMarketplaceExtension
    ;
}
