import { parseArgs } from './parse_args.ts';
import { versionForCode } from './utils.ts';

const _args = parseArgs(Deno.args, {
  string: ['engine', 'file', 'platform', 'output', 'help'],
  boolean: ['prerelease', 'openvsx'],
  collect: ['name'],
});

console.log(_args);
if (
  !_args.file ||
  !_args.engine ||
  !_args.platform ||
  _args.name.length === 0 ||
  _args.help
) {
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

const args = {
  file: _args.file!,
  engine: _args.engine!,
  platform: _args.platform!,
  output: _args.output || null,
  name: _args.name as string[],
  pre_release: _args.prerelease === true,
  is_openvsx: _args.openvsx,
};

const content = await Deno.readTextFile(args.file);
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
  await Deno.writeTextFile(args.output, yata);
} else {
  console.log(JSON.stringify(yata));
}
