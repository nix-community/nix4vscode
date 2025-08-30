{
  publisher,
  name,
  version,
  platform ? null,
  isOpenVsx,
}:
let
  platformSuffix =
    if platform == null || platform == "" then
      ""
    else if isOpenVsx then
      "@${platform}"
    else
      "targetPlatform=${platform}";
  platformInfix = if platform == null || platform == "" then "" else "/${platform}";
  extName = "${publisher}.${name}";
in
if isOpenVsx then
  "https://open-vsx.org/api/${publisher}/${name}${platformInfix}/${version}/file/${extName}-${version}${platformSuffix}.vsix"
else
  "https://${publisher}.gallery.vsassets.io/_apis/public/gallery/publisher/${publisher}/extension/${name}/${version}/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage?${platformSuffix}"
