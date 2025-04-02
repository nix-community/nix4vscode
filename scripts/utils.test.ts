import { assertEquals, assertNotEquals } from 'jsr:@std/assert';
import type { MarketplaceJson } from './types.ts';
import {
  getExtensionName,
  getExtensionVersion,
  versionForCode,
} from './utils.ts';

Deno.test('test name', () => {
  const testcase = [
    ['cpptools', 'cpptools'],
    ['ms-vscode.cpptools', 'ms-vscode.cpptools'],
    ['ms-vscode.cpptools.1.23.5', 'ms-vscode.cpptools'],
  ];
  for (const [i, o] of testcase) {
    assertEquals(o, getExtensionName(i));
  }
});

Deno.test('test_version', () => {
  const testcase = [
    ['cpptools', ''],
    ['ms-vscode.cpptools', ''],
    ['ms-vscode.cpptools.1.23.5', '1.23.5'],
  ];
  for (const [i, o] of testcase) {
    assertEquals(o, getExtensionVersion(i));
  }
});

Deno.test('test_version', async () => {
  const content = await Deno.readTextFile('../data/extensions.json');
  const data = JSON.parse(content) as MarketplaceJson;
  const x = versionForCode(
    data,
    ['continue.Continue'],
    false,
    'aarch64-darwin',
    false,
    '1.89.0',
  );
  assertNotEquals(Object.entries(x).length, 0);
});
