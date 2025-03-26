import { parseArgs } from './parse_args.ts';
import { getExtensionName, getExtensionVersion, versionBe } from './utils.ts';
import { isVersionValid } from './version.ts';

const _args = parseArgs(Deno.args, {
  string: ['engine', 'file', 'platform', 'output', 'help'],
  collect: ['name'],
});

if (
  !_args.file ||
  !_args.engine ||
  !_args.platform ||
  _args.name.length === 0 ||
  _args.help
) {
  console.log(`
Usage deno run main.ts <args> "ms-vscode.cpptools" "ms-vscode.copilot-mermaid-diagram.0.0.3"

Args:
--file: target to extensions.json
--engine: Vscode Engine
--platform: 'x86_64-linux'| 'i686-linux'| 'aarch64-linux' | 'armv7l-linux' | 'x86_64-darwin' | 'aarch64-darwin'
--output?: writer output to file.
`);
  Deno.exit(0);
}

const args = {
  file: _args.file!,
  engine: _args.engine!,
  platform: _args.platform!,
  output: _args.output || null,
  name: _args.name as string[],
};

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
const data = JSON.parse(content) as MarketplaceJson;

const plainNames = args.name.map(getExtensionName);
const nameVersion: NameVersion = {};
args.name.forEach(name => {
  nameVersion[getExtensionName(name)] = getExtensionVersion(name);
});

const x = Object.fromEntries(
  Object.entries(data)
    .filter(([name]) => {
      return plainNames.includes(name);
    })
    .map(([key, value]) => {
      const maxValue = value
        .filter(item => {
          const version = nameVersion[key];
          if (version !== '' && item.v !== version) {
            return false;
          }
          return (
            item.platform === undefined ||
            (platforms.includes(item.platform) &&
              isVersionValid(args.engine!, undefined, item.v))
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

const yata = JSON.stringify(x);
if (args.output) {
  await Deno.writeTextFile(args.output, yata);
} else {
  console.log(JSON.stringify(yata));
}
