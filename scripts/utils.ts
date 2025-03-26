import { normalizeVersion, parseVersion } from './version.ts';

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
