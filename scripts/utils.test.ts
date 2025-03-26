import { assertEquals } from 'jsr:@std/assert';
import { getExtensionName, getExtensionVersion } from './utils.ts';

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
