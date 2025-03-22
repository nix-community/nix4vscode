import { parseArgs } from 'jsr:@std/cli/parse-args';
// deno-lint-ignore-file
import { parse } from 'jsr:@std/toml';
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
  name: string;
  version: string;
  engine: string;
  platform?: string;
  // biome-ignore lint/style/useNamingConvention: <explanation>
  assert_url: string;
  hash: string;
}

const data = parse(content).extension as Marketplace[];

const filteredData = data.filter(item => {
  return (
    args.name.includes(item.name) &&
    (item.platform === undefined || platforms.includes(item.platform)) &&
    isVersionValid(args.engine!, undefined, item.engine)
  );
});

const v = Object.groupBy(filteredData, ({ name }) => {
  return name;
});

const x = Object.fromEntries(
  Object.entries(v).map(([key, value]) => {
    // biome-ignore lint/style/noNonNullAssertion: <explanation>
    const maxValue = value!.reduce((l, r) => {
      const lv = normalizeVersion(parseVersion(l.version));
      const lr = normalizeVersion(parseVersion(r.version));

      if (lv == null || lr == null) {
        return l;
      }

      if (
        lv?.majorBase > lr?.majorBase ||
        lv?.minorBase > lr?.minorBase ||
        lv?.patchBase > lr?.patchBase
      ) {
        return l;
      }

      return r;
    });
    return [key, maxValue];
  }),
);

console.log(JSON.stringify(x));
