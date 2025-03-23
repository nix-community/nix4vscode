import { parseArgs } from 'jsr:@std/cli/parse-args';
import { isVersionValid, normalizeVersion, parseVersion } from './version.ts';

const args = parseArgs(Deno.args, {
  string: ['engine', 'file', 'platform'],
  collect: ['name'],
});

if (
  args.file === undefined ||
  args.engine === undefined ||
  args.name.length === 0 ||
  args.platform === undefined
) {
  Deno.exit(-1);
}

let platforms: string[] = [];
if (args.platform === 'x86_64-linux' || args.platform === 'i686-linux') {
  platforms = ['linux-x64'];
} else if (args.platform === 'aarch64-linux') {
  platforms = ['linux-arm64'];
} else if (args.platform === 'armv7l-linux') {
  platforms = ['linux-armhf'];
} else if (args.platform === 'x86_64-darwin') {
  platforms = ['darwin-x64'];
} else if (args.platform === 'aarch64-darwin') {
  platforms = ['darwin-arm64'];
} else {
  platforms = [];
}

const content = await Deno.readTextFile(args.file);

interface Marketplace {
  n: string;
  v: string;
  e: string;
  platform?: string;
  u: string;
  h: string;
}

interface MarketplaceJson {
  [key: string]: Marketplace[];
}

const data = JSON.parse(content) as MarketplaceJson;

const v = Object.fromEntries(
  Object.entries(data).filter(([key]) => {
    return args.name.includes(key);
  }),
);

function versionBe(l: string, r: string) {
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

const x = Object.fromEntries(
  Object.entries(v).map(([key, value]) => {
    // biome-ignore lint/style/noNonNullAssertion: <explanation>
    const maxValue = value!
      .filter(item => {
        return (
          item.platform === undefined ||
          platforms.includes(item.platform) ||
          isVersionValid(args.engine!, undefined, item.v)
        );
      })
      .reduce((l, r) => {
        if (versionBe(l.v, r.v)) {
          return l;
        }

        return r;
      });
    return [key, maxValue];
  }),
);
console.log(JSON.stringify(x));
