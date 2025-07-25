import { useQuery } from '@tanstack/react-query';
import type { DataSource, ExtensionData } from '@/types/index';

const PRODUCTION_SOURCES = {
  vscode:
    'https://api.github.com/repos/nix-community/nix4vscode/contents/data/extensions.json',
  openvsx:
    'https://api.github.com/repos/nix-community/nix4vscode/contents/data/extensions_openvsx.json',
};

const isDevelopment = import.meta.env.DEV;

const DEVELOPMENT_SOURCES = isDevelopment
  ? {
      vscode: await import('../../../data/extensions.json'),
      openvsx: await import('../../../data/extensions_openvsx.json'),
    }
  : {
      vscode: undefined,
      openvsx: undefined,
    };

async function fetchExtensionData(
  dataSource: DataSource,
): Promise<ExtensionData> {
  if (isDevelopment) {
    const modulePath = DEVELOPMENT_SOURCES[dataSource];

    // biome-ignore lint/style/noNonNullAssertion: noNonNullAssertion
    const data = modulePath!.default;
    return data as ExtensionData;
  }

  // Fetch from remote in production
  const url = PRODUCTION_SOURCES[dataSource];
  console.log(`Fetching ${dataSource} data from ${url}`);

  const response = await fetch(url, {
    headers: {
      Accept: 'application/vnd.github.v3.raw',
      'Accept-Encoding': 'gzip',
    },
  });
  if (!response.ok) {
    throw new Error(
      `Failed to fetch ${dataSource} data: ${response.statusText}`,
    );
  }

  const data = await response.json();
  return data as ExtensionData;
}

export function useExtensionData(dataSource: DataSource) {
  return useQuery({
    queryKey: ['extensions', dataSource],
    queryFn: () => fetchExtensionData(dataSource),
    staleTime: 20 * 60 * 1000,
    gcTime: 30 * 60 * 1000,
  });
}
