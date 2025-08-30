let
  getExtensionUrl = import ../nix/getExtensionUrl.nix;
in
{
  testWithPlatform = {
    expr = getExtensionUrl {
      publisher = "extensionPublisher";
      name = "extensionName";
      version = "1.0.0";
      platform = "linux-x64";
      isOpenVsx = false;
    };
    expected = "https://extensionPublisher.gallery.vsassets.io/_apis/public/gallery/publisher/extensionPublisher/extension/extensionName/1.0.0/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage?targetPlatform=linux-x64";
  };
  testNoPlatform = {
    expr = getExtensionUrl {
      publisher = "extensionPublisher";
      name = "extensionName";
      version = "1.0.0";
      isOpenVsx = false;
    };
    expected = "https://extensionPublisher.gallery.vsassets.io/_apis/public/gallery/publisher/extensionPublisher/extension/extensionName/1.0.0/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage?";
  };
  testOpenVsxWithPlatform = {
    expr = getExtensionUrl {
      publisher = "extensionPublisher";
      name = "extensionName";
      version = "1.0.0";
      platform = "linux-x64";
      isOpenVsx = true;
    };
    expected = "https://open-vsx.org/api/extensionPublisher/extensionName/linux-x64/1.0.0/file/extensionPublisher.extensionName-1.0.0@linux-x64.vsix";
  };
  testOpenVsxNoPlatform = {
    expr = getExtensionUrl {
      publisher = "extensionPublisher";
      name = "extensionName";
      version = "1.0.0";
      isOpenVsx = true;
    };
    expected = "https://open-vsx.org/api/extensionPublisher/extensionName/1.0.0/file/extensionPublisher.extensionName-1.0.0.vsix";
  };
}
