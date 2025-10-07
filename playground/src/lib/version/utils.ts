import type { MarketplaceJson, NameVersion } from './types.ts';
import { isVersionValid, normalizeVersion, parseVersion } from './version.ts';

export function versionBe(l: string, r: string) {
  const lv = normalizeVersion(parseVersion(l));
  const rv = normalizeVersion(parseVersion(r));

  if (lv == null || rv == null) {
    return false;
  }

  if (lv?.majorBase > rv?.majorBase) {
    return true;
  }
  if (rv?.majorBase > lv?.majorBase) {
    return false;
  }

  if (lv?.minorBase > rv?.minorBase) {
    return true;
  }
  if (lv?.minorBase < rv?.minorBase) {
    return false;
  }
  if (lv?.patchBase > rv?.patchBase) {
    return true;
  }

  return false;
}

export function getExtensionName(name: string) {
  const pos = name.split('.');
  return pos.slice(0, 2).join('.');
}

export function getExtensionVersion(name: string) {
  const pos = name.split('.');
  return pos.slice(2).join('.');
}

export function getAssertUrl(
  isOpenVsx: boolean,
  publisher: string,
  name: string,
  version: string,
  platform?: string,
) {
  if (!isOpenVsx) {
    const platformSuffix =
      platform === undefined || platform.length === 0
        ? ''
        : `targetPlatform=${platform}`;

    return `https://${publisher}.gallery.vsassets.io/_apis/public/gallery/publisher/${publisher}/extension/${name}/${version}/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage?${platformSuffix}`;
  }

  const platformSuffix =
    platform === undefined || platform.length === 0 ? '' : `@${platform}`;
  const platformInfix =
    platform === undefined || platform.length === 0 ? '' : `/${platform}`;
  const extName = `${publisher}.${name}`;

  return `https://open-vsx.org/api/${publisher}/${name}${platformInfix}/${version}/file/${extName}-${version}${platformSuffix}.vsix`;
}

export function versionForCode(
  data: MarketplaceJson,
  name: string[],
  pre_release: boolean,
  platform: string,
  is_openvsx: boolean,
  engine: string,
) {
  const plainNames = name
    .map(getExtensionName)
    .map(value => value.toLowerCase());

  let platforms: string[] = [];
  if (platform === 'x86_64-linux' || platform === 'i686-linux') {
    platforms = ['linux-x64'];
  } else if (platform === 'aarch64-linux') {
    platforms = ['linux-arm64'];
  } else if (platform === 'armv7l-linux') {
    platforms = ['linux-armhf'];
  } else if (platform === 'x86_64-darwin') {
    platforms = ['darwin-x64'];
  } else if (platform === 'aarch64-darwin') {
    platforms = ['darwin-arm64'];
  } else {
    platforms = [];
  }

  const nameVersion: NameVersion = {};
  name.forEach(name => {
    nameVersion[getExtensionName(name).toLowerCase()] =
      getExtensionVersion(name);
  });

  const x = Object.fromEntries(
    Object.entries(data)
      .filter(([name]) => {
        return plainNames.includes(name.toLowerCase());
      })
      .map(([key, value]) => {
        const maxValue = value
          .filter(item => {
            const version = nameVersion[key];
            if (version !== '' && item.v !== version) {
              return false;
            }
            if (pre_release === false && item.r === true) {
              return false;
            }

            if (item.p === undefined) {
              return true;
            }

            if (!platforms.includes(item.p!)) {
              return false;
            }

            return isVersionValid(engine, undefined, item.e);
          })
          .reduce((l, r) => {
            if (versionBe(l.v, r.v)) {
              return l;
            }

            return r;
          });
        const [publisher, name] = key.split('.');
        maxValue.u =
          maxValue.u ||
          getAssertUrl(is_openvsx, publisher, name, maxValue.v, maxValue.p!);
        return [key, maxValue];
      }),
  );

  return x;
}
