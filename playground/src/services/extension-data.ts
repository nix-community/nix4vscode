import { useQuery } from '@tanstack/react-query';
import type { DataSource, ExtensionData } from '@/types/index';

const PRODUCTION_SOURCES = {
  vscode:
    'https://github.com/nix-community/nix4vscode/raw/refs/heads/master/data/extensions.json',
  openvsx:
    'https://github.com/nix-community/nix4vscode/raw/refs/heads/master/data/extensions_openvsx.json',
};

const DEVELOPMENT_SOURCES = {
  vscode: '../../../data/extensions.json',
  openvsx: '../../../data/extensions_openvsx.json',
};

const isDevelopment = import.meta.env.DEV;

async function fetchExtensionData(
  dataSource: DataSource,
): Promise<ExtensionData> {
  if (isDevelopment) {
    // Use dynamic import in development
    const modulePath = DEVELOPMENT_SOURCES[dataSource];
    console.log(`Dynamically importing ${dataSource} data from ${modulePath}`);

    const module = await import(/* @vite-ignore */ modulePath);
    const data = module.default;
    return data as ExtensionData;
  }

  // Fetch from remote in production
  const url = PRODUCTION_SOURCES[dataSource];
  console.log(`Fetching ${dataSource} data from ${url}`);

  const response = await fetch(url);
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
    staleTime: 10 * 60 * 1000, // 10 minutes
    gcTime: 30 * 60 * 1000, // 30 minutes
  });
}
