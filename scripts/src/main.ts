import * as std from 'std';

import { versionForCode } from './utils';

import { typeFlag } from 'type-flag';
import type { MarketplaceJson } from './types';

const { flags: _args } = typeFlag(
  {
    engine: String,
    file: String,
    platform: String,
    output: String,
    help: Boolean,
    prerelease: Boolean,
    openvsx: Boolean,
    name: [String],
  },
  scriptArgs,
);

if (_args.help) {
}

if (
  !_args.file ||
  !_args.engine ||
  !_args.platform ||
  _args.name.length === 0 ||
  _args.help
) {
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

const args = {
  file: _args.file!,
  engine: _args.engine!,
  platform: _args.platform!,
  output: _args.output || null,
  name: _args.name as string[],
  pre_release: _args.prerelease === true,
  is_openvsx: _args.openvsx,
};

const content = std.loadFile(args.file);
const data = JSON.parse(content) as MarketplaceJson;

const x = versionForCode(
  data,
  args.name,
  args.pre_release,
  args.platform,
  args.is_openvsx,
  args.engine,
);

const yata = JSON.stringify(x);
if (args.output) {
  const file = std.open(args.output, 'w');
  file.puts(yata);
  file.close();
} else {
  console.log(yata);
}
