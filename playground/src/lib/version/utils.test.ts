import { readFileSync } from 'node:fs';
import { expect, test } from 'vitest';
import type { MarketplaceJson } from './types.ts';
import { getExtensionName, getExtensionVersion, versionForCode } from './utils';

test('test name', () => {
  const testcase = [
    ['cpptools', 'cpptools'],
    ['ms-vscode.cpptools', 'ms-vscode.cpptools'],
    ['ms-vscode.cpptools.1.23.5', 'ms-vscode.cpptools'],
  ];
  for (const [i, o] of testcase) {
    expect(o).eq(getExtensionName(i));
  }
});

test('test_version', () => {
  const testcase = [
    ['cpptools', ''],
    ['ms-vscode.cpptools', ''],
    ['ms-vscode.cpptools.1.23.5', '1.23.5'],
  ];
  for (const [i, o] of testcase) {
    expect(o).eq(getExtensionVersion(i));
  }
});

test('test_version', async () => {
  const content = readFileSync('../data/extensions.json').toString();
  const data = JSON.parse(content) as MarketplaceJson;
  const x = versionForCode(
    data,
    ['continue.Continue'],
    false,
    'aarch64-darwin',
    false,
    '1.89.0',
  );
  expect(Object.entries(x).length).eq(1);
});
