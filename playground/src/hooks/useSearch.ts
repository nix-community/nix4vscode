import { useMemo } from 'react';
import type {
  ExtensionData,
  ExtensionItem,
  SearchFilters,
} from '@/types/index';

function versionMatches(
  extensionEngine: string,
  targetVersion: string,
): boolean {
  if (!targetVersion.trim()) return true;

  // Simple version matching - can be enhanced with semver library
  const cleanTarget = targetVersion.replace(/[\^~>=<]/g, '');
  const cleanEngine = extensionEngine.replace(/[\^~>=<]/g, '');

  return cleanEngine.includes(cleanTarget) || cleanTarget.includes(cleanEngine);
}

export function useSearch(
  data: ExtensionData | undefined,
  filters: SearchFilters,
) {
  return useMemo(() => {
    if (!data) return [];

    const extensions: ExtensionItem[] = Object.entries(data).map(
      ([name, versions]) => ({
        name,
        versions,
      }),
    );

    return extensions
      .filter(extension => {
        // Text search
        if (filters.query.trim()) {
          const query = filters.query.toLowerCase();
          if (!extension.name.toLowerCase().includes(query)) {
            return false;
          }
        }

        // Filter versions based on criteria
        const filteredVersions = extension.versions.filter(version => {
          // VSCode version compatibility
          if (filters.vscodeVersion && filters.vscodeVersion.trim()) {
            if (!versionMatches(version.e, filters.vscodeVersion)) {
              return false;
            }
          }

          // Prerelease filter
          if (!filters.includePrerelease && version.r) {
            return false;
          }

          return true;
        });

        // Only include extensions that have at least one matching version
        return filteredVersions.length > 0;
      })
      .map(extension => ({
        ...extension,
        versions: extension.versions.filter(version => {
          // Apply same filters to versions
          if (filters.vscodeVersion && filters.vscodeVersion.trim()) {
            if (!versionMatches(version.e, filters.vscodeVersion)) {
              return false;
            }
          }

          if (!filters.includePrerelease && version.r) {
            return false;
          }

          return true;
        }),
      }));
  }, [data, filters]);
}
