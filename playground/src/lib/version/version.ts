// https://github.com/microsoft/vscode/blob/747b90cfcd486731a1295563423b1ee66dd9b613/src/vs/platform/extensions/common/extensionValidator.ts

export interface IParsedVersion {
  hasCaret: boolean;
  hasGreaterEquals: boolean;
  majorBase: number;
  majorMustEqual: boolean;
  minorBase: number;
  minorMustEqual: boolean;
  patchBase: number;
  patchMustEqual: boolean;
  preRelease: string | null;
}

export interface INormalizedVersion {
  majorBase: number;
  majorMustEqual: boolean;
  minorBase: number;
  minorMustEqual: boolean;
  patchBase: number;
  patchMustEqual: boolean;
  notBefore: number /* milliseconds timestamp, or 0 */;
  isMinimum: boolean;
}

const VERSION_REGEXP = /^(\^|>=)?((\d+)|x)\.((\d+)|x)\.((\d+)|x)(-.*)?$/;
const NOT_BEFORE_REGEXP = /^-(\d{4})(\d{2})(\d{2})$/;

export function isValidVersionStr(version: string): boolean {
  version = version.trim();
  return version === '*' || VERSION_REGEXP.test(version);
}

export function parseVersion(version: string): IParsedVersion | null {
  if (!isValidVersionStr(version)) {
    return null;
  }

  version = version.trim();

  if (version === '*') {
    return {
      hasCaret: false,
      hasGreaterEquals: false,
      majorBase: 0,
      majorMustEqual: false,
      minorBase: 0,
      minorMustEqual: false,
      patchBase: 0,
      patchMustEqual: false,
      preRelease: null,
    };
  }

  const m = version.match(VERSION_REGEXP);
  if (!m) {
    return null;
  }
  return {
    hasCaret: m[1] === '^',
    hasGreaterEquals: m[1] === '>=',
    majorBase: m[2] === 'x' ? 0 : Number.parseInt(m[2], 10),
    majorMustEqual: m[2] === 'x' ? false : true,
    minorBase: m[4] === 'x' ? 0 : Number.parseInt(m[4], 10),
    minorMustEqual: m[4] === 'x' ? false : true,
    patchBase: m[6] === 'x' ? 0 : Number.parseInt(m[6], 10),
    patchMustEqual: m[6] === 'x' ? false : true,
    preRelease: m[8] || null,
  };
}

export function normalizeVersion(
  version: IParsedVersion | null,
): INormalizedVersion | null {
  if (!version) {
    return null;
  }

  const majorBase = version.majorBase;
  const majorMustEqual = version.majorMustEqual;
  const minorBase = version.minorBase;
  let minorMustEqual = version.minorMustEqual;
  const patchBase = version.patchBase;
  let patchMustEqual = version.patchMustEqual;

  if (version.hasCaret) {
    if (majorBase === 0) {
      patchMustEqual = false;
    } else {
      minorMustEqual = false;
      patchMustEqual = false;
    }
  }

  let notBefore = 0;
  if (version.preRelease) {
    const match = NOT_BEFORE_REGEXP.exec(version.preRelease);
    if (match) {
      const [, year, month, day] = match;
      notBefore = Date.UTC(Number(year), Number(month) - 1, Number(day));
    }
  }

  return {
    majorBase: majorBase,
    majorMustEqual: majorMustEqual,
    minorBase: minorBase,
    minorMustEqual: minorMustEqual,
    patchBase: patchBase,
    patchMustEqual: patchMustEqual,
    isMinimum: version.hasGreaterEquals,
    notBefore,
  };
}

export function isValidVersion(
  _inputVersion: string | INormalizedVersion,
  _inputDate: ProductDate,
  _desiredVersion: string | INormalizedVersion,
): boolean {
  let version: INormalizedVersion | null;
  if (typeof _inputVersion === 'string') {
    version = normalizeVersion(parseVersion(_inputVersion));
  } else {
    version = _inputVersion;
  }

  let productTs: number | undefined;
  if (_inputDate instanceof Date) {
    productTs = _inputDate.getTime();
  } else if (typeof _inputDate === 'string') {
    productTs = new Date(_inputDate).getTime();
  }

  let desiredVersion: INormalizedVersion | null;
  if (typeof _desiredVersion === 'string') {
    desiredVersion = normalizeVersion(parseVersion(_desiredVersion));
  } else {
    desiredVersion = _desiredVersion;
  }

  if (!version || !desiredVersion) {
    return false;
  }

  const majorBase = version.majorBase;
  const minorBase = version.minorBase;
  const patchBase = version.patchBase;

  let desiredMajorBase = desiredVersion.majorBase;
  let desiredMinorBase = desiredVersion.minorBase;
  let desiredPatchBase = desiredVersion.patchBase;
  const desiredNotBefore = desiredVersion.notBefore;

  let majorMustEqual = desiredVersion.majorMustEqual;
  let minorMustEqual = desiredVersion.minorMustEqual;
  let patchMustEqual = desiredVersion.patchMustEqual;

  if (desiredVersion.isMinimum) {
    if (majorBase > desiredMajorBase) {
      return true;
    }

    if (majorBase < desiredMajorBase) {
      return false;
    }

    if (minorBase > desiredMinorBase) {
      return true;
    }

    if (minorBase < desiredMinorBase) {
      return false;
    }

    if (productTs && productTs < desiredNotBefore) {
      return false;
    }

    return patchBase >= desiredPatchBase;
  }

  // Anything < 1.0.0 is compatible with >= 1.0.0, except exact matches
  if (
    majorBase === 1 &&
    desiredMajorBase === 0 &&
    (!majorMustEqual || !minorMustEqual || !patchMustEqual)
  ) {
    desiredMajorBase = 1;
    desiredMinorBase = 0;
    desiredPatchBase = 0;
    majorMustEqual = true;
    minorMustEqual = false;
    patchMustEqual = false;
  }

  if (majorBase < desiredMajorBase) {
    // smaller major version
    return false;
  }

  if (majorBase > desiredMajorBase) {
    // higher major version
    return !majorMustEqual;
  }

  // at this point, majorBase are equal

  if (minorBase < desiredMinorBase) {
    // smaller minor version
    return false;
  }

  if (minorBase > desiredMinorBase) {
    // higher minor version
    return !minorMustEqual;
  }

  // at this point, minorBase are equal

  if (patchBase < desiredPatchBase) {
    // smaller patch version
    return false;
  }

  if (patchBase > desiredPatchBase) {
    // higher patch version
    return !patchMustEqual;
  }

  // at this point, patchBase are equal

  if (productTs && productTs < desiredNotBefore) {
    return false;
  }

  return true;
}

type ProductDate = string | Date | undefined;

export function isEngineValid(
  engine: string,
  version: string,
  date: ProductDate,
): boolean {
  // TODO@joao: discuss with alex '*' doesn't seem to be a valid engine version
  return engine === '*' || isVersionValid(version, date, engine);
}

export function isVersionValid(
  currentVersion: string,
  date: ProductDate,
  requestedVersion: string,
): boolean {
  const desiredVersion = normalizeVersion(parseVersion(requestedVersion));
  if (!desiredVersion) {
    return false;
  }

  // enforce that a breaking API version is specified.
  // for 0.X.Y, that means up to 0.X must be specified
  // otherwise for Z.X.Y, that means Z must be specified
  if (desiredVersion.majorBase === 0) {
    // force that major and minor must be specific
    if (!desiredVersion.majorMustEqual || !desiredVersion.minorMustEqual) {
      return false;
    }
  } else {
    // force that major must be specific
    if (!desiredVersion.majorMustEqual) {
      return false;
    }
  }

  if (!isValidVersion(currentVersion, date, desiredVersion)) {
    return false;
  }

  return true;
}
