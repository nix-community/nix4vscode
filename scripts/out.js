// parse_args.ts
var FLAG_REGEXP = /^(?:-(?:(?<doubleDash>-)(?<negated>no-)?)?)(?<key>.+?)(?:=(?<value>.+?))?$/s;
var LETTER_REGEXP = /[A-Za-z]/;
var NUMBER_REGEXP = /-?\d+(\.\d*)?(e-?\d+)?$/;
var HYPHEN_REGEXP = /^(-|--)[^-]/;
var VALUE_REGEXP = /=(?<value>.+)/;
var FLAG_NAME_REGEXP = /^--[^=]+$/;
var SPECIAL_CHAR_REGEXP = /\W/;
var NON_WHITESPACE_REGEXP = /\S/;
function isNumber(string) {
  return NON_WHITESPACE_REGEXP.test(string) && Number.isFinite(Number(string));
}
function setNested(object, keys, value, collect = false) {
  keys = [...keys];
  const key = keys.pop();
  keys.forEach((key2) => object = object[key2] ??= {});
  if (collect) {
    const v = object[key];
    if (Array.isArray(v)) {
      v.push(value);
      return;
    }
    value = v ? [v, value] : [value];
  }
  object[key] = value;
}
function hasNested(object, keys) {
  for (const key of keys) {
    const value = object[key];
    if (!Object.hasOwn(object, key)) return false;
    object = value;
  }
  return true;
}
function aliasIsBoolean(aliasMap, booleanSet, key) {
  const set = aliasMap.get(key);
  if (set === void 0) return false;
  for (const alias of set) if (booleanSet.has(alias)) return true;
  return false;
}
function isBooleanString(value) {
  return value === "true" || value === "false";
}
function parseBooleanString(value) {
  return value !== "false";
}
function parseArgs(args2, options) {
  const {
    "--": doubleDash = false,
    alias = {},
    boolean = false,
    default: defaults = {},
    stopEarly = false,
    string = [],
    collect = [],
    negatable = [],
    unknown: unknownFn = (i) => i
  } = options ?? {};
  const aliasMap = /* @__PURE__ */ new Map();
  const booleanSet = /* @__PURE__ */ new Set();
  const stringSet = /* @__PURE__ */ new Set();
  const collectSet = /* @__PURE__ */ new Set();
  const negatableSet = /* @__PURE__ */ new Set();
  let allBools = false;
  if (alias) {
    for (const [key, value] of Object.entries(alias)) {
      if (value === void 0) {
        throw new TypeError("Alias value must be defined");
      }
      const aliases = Array.isArray(value) ? value : [value];
      aliasMap.set(key, new Set(aliases));
      aliases.forEach(
        (alias2) => aliasMap.set(
          alias2,
          /* @__PURE__ */ new Set([key, ...aliases.filter((it) => it !== alias2)])
        )
      );
    }
  }
  if (boolean) {
    if (typeof boolean === "boolean") {
      allBools = boolean;
    } else {
      const booleanArgs = Array.isArray(boolean) ? boolean : [boolean];
      for (const key of booleanArgs.filter(Boolean)) {
        booleanSet.add(key);
        aliasMap.get(key)?.forEach((al) => {
          booleanSet.add(al);
        });
      }
    }
  }
  if (string) {
    const stringArgs = Array.isArray(string) ? string : [string];
    for (const key of stringArgs.filter(Boolean)) {
      stringSet.add(key);
      aliasMap.get(key)?.forEach((al) => stringSet.add(al));
    }
  }
  if (collect) {
    const collectArgs = Array.isArray(collect) ? collect : [collect];
    for (const key of collectArgs.filter(Boolean)) {
      collectSet.add(key);
      aliasMap.get(key)?.forEach((al) => collectSet.add(al));
    }
  }
  if (negatable) {
    const negatableArgs = Array.isArray(negatable) ? negatable : [negatable];
    for (const key of negatableArgs.filter(Boolean)) {
      negatableSet.add(key);
      aliasMap.get(key)?.forEach((alias2) => negatableSet.add(alias2));
    }
  }
  const argv = { _: [] };
  function setArgument(key, value, arg, collect2) {
    if (!booleanSet.has(key) && !stringSet.has(key) && !aliasMap.has(key) && !(allBools && FLAG_NAME_REGEXP.test(arg)) && unknownFn?.(arg, key, value) === false) {
      return;
    }
    if (typeof value === "string" && !stringSet.has(key)) {
      value = isNumber(value) ? Number(value) : value;
    }
    const collectable = collect2 && collectSet.has(key);
    setNested(argv, key.split("."), value, collectable);
    aliasMap.get(key)?.forEach((key2) => {
      setNested(argv, key2.split("."), value, collectable);
    });
  }
  let notFlags = [];
  const index = args2.indexOf("--");
  if (index !== -1) {
    notFlags = args2.slice(index + 1);
    args2 = args2.slice(0, index);
  }
  argsLoop: for (let i = 0; i < args2.length; i++) {
    const arg = args2[i];
    const groups = arg.match(FLAG_REGEXP)?.groups;
    if (groups) {
      const { doubleDash: doubleDash2, negated } = groups;
      let key = groups.key;
      let value = groups.value;
      if (doubleDash2) {
        if (value) {
          if (booleanSet.has(key)) value = parseBooleanString(value);
          setArgument(key, value, arg, true);
          continue;
        }
        if (negated) {
          if (negatableSet.has(key)) {
            setArgument(key, false, arg, false);
            continue;
          }
          key = `no-${key}`;
        }
        const next = args2[i + 1];
        if (next) {
          if (!booleanSet.has(key) && !allBools && !next.startsWith("-") && (!aliasMap.has(key) || !aliasIsBoolean(aliasMap, booleanSet, key))) {
            value = next;
            i++;
            setArgument(key, value, arg, true);
            continue;
          }
          if (isBooleanString(next)) {
            value = parseBooleanString(next);
            i++;
            setArgument(key, value, arg, true);
            continue;
          }
        }
        value = stringSet.has(key) ? "" : true;
        setArgument(key, value, arg, true);
        continue;
      }
      const letters = arg.slice(1, -1).split("");
      for (const [j, letter] of letters.entries()) {
        const next = arg.slice(j + 2);
        if (next === "-") {
          setArgument(letter, next, arg, true);
          continue;
        }
        if (LETTER_REGEXP.test(letter)) {
          const groups2 = VALUE_REGEXP.exec(next)?.groups;
          if (groups2) {
            setArgument(letter, groups2.value, arg, true);
            continue argsLoop;
          }
          if (NUMBER_REGEXP.test(next)) {
            setArgument(letter, next, arg, true);
            continue argsLoop;
          }
        }
        if (letters[j + 1]?.match(SPECIAL_CHAR_REGEXP)) {
          setArgument(letter, arg.slice(j + 2), arg, true);
          continue argsLoop;
        }
        setArgument(letter, stringSet.has(letter) ? "" : true, arg, true);
      }
      key = arg.slice(-1);
      if (key === "-") continue;
      const nextArg = args2[i + 1];
      if (nextArg) {
        if (!HYPHEN_REGEXP.test(nextArg) && !booleanSet.has(key) && (!aliasMap.has(key) || !aliasIsBoolean(aliasMap, booleanSet, key))) {
          setArgument(key, nextArg, arg, true);
          i++;
          continue;
        }
        if (isBooleanString(nextArg)) {
          const value2 = parseBooleanString(nextArg);
          setArgument(key, value2, arg, true);
          i++;
          continue;
        }
      }
      setArgument(key, stringSet.has(key) ? "" : true, arg, true);
      continue;
    }
    if (unknownFn?.(arg) !== false) {
      argv._.push(stringSet.has("_") || !isNumber(arg) ? arg : Number(arg));
    }
    if (stopEarly) {
      argv._.push(...args2.slice(i + 1));
      break;
    }
  }
  for (const [key, value] of Object.entries(defaults)) {
    const keys = key.split(".");
    if (!hasNested(argv, keys)) {
      setNested(argv, keys, value);
      aliasMap.get(key)?.forEach((key2) => setNested(argv, key2.split("."), value));
    }
  }
  for (const key of booleanSet.keys()) {
    const keys = key.split(".");
    if (!hasNested(argv, keys)) {
      const value = collectSet.has(key) ? [] : false;
      setNested(argv, keys, value);
    }
  }
  for (const key of stringSet.keys()) {
    const keys = key.split(".");
    if (!hasNested(argv, keys) && collectSet.has(key)) {
      setNested(argv, keys, []);
    }
  }
  if (doubleDash) {
    argv["--"] = notFlags;
  } else {
    argv._.push(...notFlags);
  }
  return argv;
}

// version.ts
var VERSION_REGEXP = /^(\^|>=)?((\d+)|x)\.((\d+)|x)\.((\d+)|x)(\-.*)?$/;
var NOT_BEFORE_REGEXP = /^-(\d{4})(\d{2})(\d{2})$/;
function isValidVersionStr(version) {
  version = version.trim();
  return version === "*" || VERSION_REGEXP.test(version);
}
function parseVersion(version) {
  if (!isValidVersionStr(version)) {
    return null;
  }
  version = version.trim();
  if (version === "*") {
    return {
      hasCaret: false,
      hasGreaterEquals: false,
      majorBase: 0,
      majorMustEqual: false,
      minorBase: 0,
      minorMustEqual: false,
      patchBase: 0,
      patchMustEqual: false,
      preRelease: null
    };
  }
  const m = version.match(VERSION_REGEXP);
  if (!m) {
    return null;
  }
  return {
    hasCaret: m[1] === "^",
    hasGreaterEquals: m[1] === ">=",
    majorBase: m[2] === "x" ? 0 : Number.parseInt(m[2], 10),
    majorMustEqual: m[2] === "x" ? false : true,
    minorBase: m[4] === "x" ? 0 : Number.parseInt(m[4], 10),
    minorMustEqual: m[4] === "x" ? false : true,
    patchBase: m[6] === "x" ? 0 : Number.parseInt(m[6], 10),
    patchMustEqual: m[6] === "x" ? false : true,
    preRelease: m[8] || null
  };
}
function normalizeVersion(version) {
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
    majorBase,
    majorMustEqual,
    minorBase,
    minorMustEqual,
    patchBase,
    patchMustEqual,
    isMinimum: version.hasGreaterEquals,
    notBefore
  };
}
function isValidVersion(_inputVersion, _inputDate, _desiredVersion) {
  let version;
  if (typeof _inputVersion === "string") {
    version = normalizeVersion(parseVersion(_inputVersion));
  } else {
    version = _inputVersion;
  }
  let productTs;
  if (_inputDate instanceof Date) {
    productTs = _inputDate.getTime();
  } else if (typeof _inputDate === "string") {
    productTs = new Date(_inputDate).getTime();
  }
  let desiredVersion;
  if (typeof _desiredVersion === "string") {
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
  if (majorBase === 1 && desiredMajorBase === 0 && (!majorMustEqual || !minorMustEqual || !patchMustEqual)) {
    desiredMajorBase = 1;
    desiredMinorBase = 0;
    desiredPatchBase = 0;
    majorMustEqual = true;
    minorMustEqual = false;
    patchMustEqual = false;
  }
  if (majorBase < desiredMajorBase) {
    return false;
  }
  if (majorBase > desiredMajorBase) {
    return !majorMustEqual;
  }
  if (minorBase < desiredMinorBase) {
    return false;
  }
  if (minorBase > desiredMinorBase) {
    return !minorMustEqual;
  }
  if (patchBase < desiredPatchBase) {
    return false;
  }
  if (patchBase > desiredPatchBase) {
    return !patchMustEqual;
  }
  if (productTs && productTs < desiredNotBefore) {
    return false;
  }
  return true;
}
function isVersionValid(currentVersion, date, requestedVersion) {
  const desiredVersion = normalizeVersion(parseVersion(requestedVersion));
  if (!desiredVersion) {
    return false;
  }
  if (desiredVersion.majorBase === 0) {
    if (!desiredVersion.majorMustEqual || !desiredVersion.minorMustEqual) {
      return false;
    }
  } else {
    if (!desiredVersion.majorMustEqual) {
      return false;
    }
  }
  if (!isValidVersion(currentVersion, date, desiredVersion)) {
    return false;
  }
  return true;
}

// utils.ts
function versionBe(l, r) {
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
function getExtensionName(name) {
  const pos = name.split(".");
  return pos.slice(0, 2).join(".");
}
function getExtensionVersion(name) {
  const pos = name.split(".");
  return pos.slice(2).join(".");
}
function getAssertUrl(isOpenVsx, publisher, name, version, platform) {
  const platformSuffix = platform === void 0 || platform.length === 0 ? "" : `targetPlatform=${platform}`;
  if (!isOpenVsx) {
    return `https://${publisher}.gallery.vsassets.io/_apis/public/gallery/publisher/${publisher}/extension/${name}/${version}/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage?${platformSuffix}`;
  }
  const platformInfix = platform === void 0 || platform.length === 0 ? "" : `/${platform}`;
  const extName = `${publisher}.${name}`;
  return `https://open-vsx.org/api/${publisher}/${name}${platformInfix}/${version}/file/${extName}-${version}${platformSuffix}.vsix`;
}
function versionForCode(data2, name, pre_release, platform, is_openvsx, engine) {
  const plainNames = name.map(getExtensionName).map((value) => value.toLowerCase());
  let platforms = [];
  if (platform === "x86_64-linux" || platform === "i686-linux") {
    platforms = ["linux-x64"];
  } else if (platform === "aarch64-linux") {
    platforms = ["linux-arm64"];
  } else if (platform === "armv7l-linux") {
    platforms = ["linux-armhf"];
  } else if (platform === "x86_64-darwin") {
    platforms = ["darwin-x64"];
  } else if (platform === "aarch64-darwin") {
    platforms = ["darwin-arm64"];
  } else {
    platforms = [];
  }
  const nameVersion = {};
  name.forEach((name2) => {
    nameVersion[getExtensionName(name2).toLowerCase()] = getExtensionVersion(name2);
  });
  const x2 = Object.fromEntries(
    Object.entries(data2).filter(([name2]) => {
      return plainNames.includes(name2.toLowerCase());
    }).map(([key, value]) => {
      const maxValue = value.filter((item) => {
        const version = nameVersion[key];
        if (version !== "" && item.v !== version) {
          return false;
        }
        if (pre_release === false && item.r === true) {
          return false;
        }
        if (item.p === void 0) {
          return true;
        }
        if (!platforms.includes(item.p)) {
          return false;
        }
        return isVersionValid(engine, void 0, item.e);
      }).reduce((l, r) => {
        if (versionBe(l.v, r.v)) {
          return l;
        }
        return r;
      });
      const [publisher, name2] = key.split(".");
      maxValue.u = maxValue.u || getAssertUrl(is_openvsx, publisher, name2, maxValue.v, maxValue.p);
      return [key, maxValue];
    })
  );
  return x2;
}

// main.ts
var _args = parseArgs(Deno.args, {
  string: ["engine", "file", "platform", "output", "help"],
  boolean: ["prerelease", "openvsx"],
  collect: ["name"]
});
console.log(_args);
if (!_args.file || !_args.engine || !_args.platform || _args.name.length === 0 || _args.help) {
  console.log(`
Usage deno run main.ts <args> --name "ms-vscode.cpptools" --name "ms-vscode.copilot-mermaid-diagram.0.0.3"

Args:
--file: target to extensions.json
--engine: Vscode Engine
--platform: 'x86_64-linux'| 'i686-linux'| 'aarch64-linux' | 'armv7l-linux' | 'x86_64-darwin' | 'aarch64-darwin'
--output?: writer output to file.
`);
  Deno.exit(0);
}
var args = {
  file: _args.file,
  engine: _args.engine,
  platform: _args.platform,
  output: _args.output || null,
  name: _args.name,
  pre_release: _args.prerelease === true,
  is_openvsx: _args.openvsx
};
var content = await Deno.readTextFile(args.file);
var data = JSON.parse(content);
var x = versionForCode(
  data,
  args.name,
  args.pre_release,
  args.platform,
  args.is_openvsx,
  args.engine
);
var yata = JSON.stringify(x);
if (args.output) {
  await Deno.writeTextFile(args.output, yata);
} else {
  console.log(JSON.stringify(yata));
}
