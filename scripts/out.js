// src/main.ts
import * as std from "std";

// src/version.ts
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
  const m2 = version.match(VERSION_REGEXP);
  if (!m2) {
    return null;
  }
  return {
    hasCaret: m2[1] === "^",
    hasGreaterEquals: m2[1] === ">=",
    majorBase: m2[2] === "x" ? 0 : Number.parseInt(m2[2], 10),
    majorMustEqual: m2[2] === "x" ? false : true,
    minorBase: m2[4] === "x" ? 0 : Number.parseInt(m2[4], 10),
    minorMustEqual: m2[4] === "x" ? false : true,
    patchBase: m2[6] === "x" ? 0 : Number.parseInt(m2[6], 10),
    patchMustEqual: m2[6] === "x" ? false : true,
    preRelease: m2[8] || null
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

// src/utils.ts
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

// node_modules/.pnpm/type-flag@3.0.0/node_modules/type-flag/dist/index.mjs
var V = "known-flag";
var k = "unknown-flag";
var C = "argument";
var { stringify: h } = JSON;
var O = /\B([A-Z])/g;
var v = (t) => t.replace(O, "-$1").toLowerCase();
var { hasOwnProperty: D } = Object.prototype;
var w = (t, n) => D.call(t, n);
var L = (t) => Array.isArray(t);
var b = (t) => typeof t == "function" ? [t, false] : L(t) ? [t[0], true] : b(t.type);
var d = (t, n) => t === Boolean ? n !== "false" : n;
var m = (t, n) => typeof n == "boolean" ? n : t === Number && n === "" ? Number.NaN : t(n);
var R = /[\s.:=]/;
var B = (t) => {
  const n = `Flag name ${h(t)}`;
  if (t.length === 0) throw new Error(`${n} cannot be empty`);
  if (t.length === 1) throw new Error(`${n} must be longer than a character`);
  const r = t.match(R);
  if (r) throw new Error(`${n} cannot contain ${h(r?.[0])}`);
};
var K = (t) => {
  const n = {}, r = (e, o) => {
    if (w(n, e)) throw new Error(`Duplicate flags named ${h(e)}`);
    n[e] = o;
  };
  for (const e in t) {
    if (!w(t, e)) continue;
    B(e);
    const o = t[e], s = [[], ...b(o), o];
    r(e, s);
    const i = v(e);
    if (e !== i && r(i, s), "alias" in o && typeof o.alias == "string") {
      const { alias: a } = o, l = `Flag alias ${h(a)} for flag ${h(e)}`;
      if (a.length === 0) throw new Error(`${l} cannot be empty`);
      if (a.length > 1) throw new Error(`${l} must be a single character`);
      r(a, s);
    }
  }
  return n;
};
var _ = (t, n) => {
  const r = {};
  for (const e in t) {
    if (!w(t, e)) continue;
    const [o, , s, i] = n[e];
    if (o.length === 0 && "default" in i) {
      let { default: a } = i;
      typeof a == "function" && (a = a()), r[e] = a;
    } else r[e] = s ? o : o.pop();
  }
  return r;
};
var F = "--";
var G = /[.:=]/;
var T = /^-{1,2}\w/;
var N = (t) => {
  if (!T.test(t)) return;
  const n = !t.startsWith(F);
  let r = t.slice(n ? 1 : 2), e;
  const o = r.match(G);
  if (o) {
    const { index: s } = o;
    e = r.slice(s + 1), r = r.slice(0, s);
  }
  return [r, e, n];
};
var $ = (t, { onFlag: n, onArgument: r }) => {
  let e;
  const o = (s, i) => {
    if (typeof e != "function") return true;
    e(s, i), e = void 0;
  };
  for (let s = 0; s < t.length; s += 1) {
    const i = t[s];
    if (i === F) {
      o();
      const l = t.slice(s + 1);
      r?.(l, [s], true);
      break;
    }
    const a = N(i);
    if (a) {
      if (o(), !n) continue;
      const [l, f, g] = a;
      if (g) for (let c = 0; c < l.length; c += 1) {
        o();
        const u = c === l.length - 1;
        e = n(l[c], u ? f : void 0, [s, c + 1, u]);
      }
      else e = n(l, f, [s]);
    } else o(i, [s]) && r?.([i], [s]);
  }
  o();
};
var E = (t, n) => {
  for (const [r, e, o] of n.reverse()) {
    if (e) {
      const s = t[r];
      let i = s.slice(0, e);
      if (o || (i += s.slice(e + 1)), i !== "-") {
        t[r] = i;
        continue;
      }
    }
    t.splice(r, 1);
  }
};
var U = (t, n = process.argv.slice(2), { ignore: r } = {}) => {
  const e = [], o = K(t), s = {}, i = [];
  return i[F] = [], $(n, { onFlag(a, l, f) {
    const g = w(o, a);
    if (!r?.(g ? V : k, a, l)) {
      if (g) {
        const [c, u] = o[a], y = d(u, l), p = (P, A) => {
          e.push(f), A && e.push(A), c.push(m(u, P || ""));
        };
        return y === void 0 ? p : p(y);
      }
      w(s, a) || (s[a] = []), s[a].push(l === void 0 ? true : l), e.push(f);
    }
  }, onArgument(a, l, f) {
    r?.(C, n[l[0]]) || (i.push(...a), f ? (i[F] = a, n.splice(l[0])) : e.push(l));
  } }), E(n, e), { flags: _(t, o), unknownFlags: s, _: i };
};

// src/main.ts
var { flags: _args } = U(
  {
    engine: String,
    file: String,
    platform: String,
    output: String,
    help: Boolean,
    prerelease: Boolean,
    openvsx: Boolean,
    name: [String]
  },
  scriptArgs
);
if (_args.help) {
}
if (!_args.file || !_args.engine || !_args.platform || _args.name.length === 0 || _args.help) {
  console.log(`
Usage:
  qjs script.js [options]

Options:
  --engine <version>        VSCode version string, e.g. "1.101.2"
  --file <file>             Input source file path
  --platform <platform>     Target platform, one of:
                            'x86_64-linux', 'i686-linux', 'aarch64-linux',
                            'armv7l-linux', 'x86_64-darwin', 'aarch64-darwin'
  --output <path>           Output bundle path
  --prerelease              Mark version as a prerelease
  --openvsx                 Enable publishing to Open VSX
  --name <name>             Component name (can be repeated)
  --help                    Show this help message

Example:
  qjs out.js --file ../data/extensions.json --platform aarch64-darwin \\
    --engine 1.101.2 --name "ms-vscode.cpptools"
`);
  std.exit(0);
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
var content = std.loadFile(args.file);
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
  const file = std.open(args.output, "w");
  file.puts(yata);
  file.close();
} else {
  console.log(yata);
}
