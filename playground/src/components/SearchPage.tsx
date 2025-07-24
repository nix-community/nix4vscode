import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { useState } from 'react';
import { Toaster } from '@/components/ui/sonner';
import { useSearch } from '@/hooks/useSearch';
import { queryClient } from '@/lib/query-client';
import { useExtensionData } from '@/services/extension-data';
import type { DataSource, SearchFilters } from '@/types/index';
import { Button } from './ui/button';
import { Checkbox } from './ui/checkbox';
import { Input } from './ui/input';
import { Label } from './ui/label';
import VirtualExtensionList from './VirtualExtensionList';

function SearchContent() {
  const [filters, setFilters] = useState<SearchFilters>({
    query: '',
    dataSource: 'vscode',
    vscodeVersion: '',
    includePrerelease: true,
  });

  const updateFilter = <K extends keyof SearchFilters>(
    key: K,
    value: SearchFilters[K],
  ) => {
    setFilters(prev => ({ ...prev, [key]: value }));
  };

  const { data, isLoading, error } = useExtensionData(filters.dataSource);
  const filteredExtensions = useSearch(data, filters);

  return (
    <div className="min-h-screen bg-background">
      <div className="container mx-auto px-4 py-8">
        {/* Header Section */}
        <div className="mb-8 text-center">
          <h1 className="mb-4 font-bold text-4xl">Extension Search</h1>
          <p className="text-muted-foreground">
            Search VSCode and OpenVSX extensions
          </p>
        </div>

        {/* Search Controls */}
        <div className="mx-auto mb-8 max-w-4xl">
          <div className="space-y-4 rounded-lg border bg-card p-6">
            {/* Data Source Selector */}
            <div className="flex flex-col items-start gap-4 sm:flex-row sm:items-center">
              <span className="min-w-fit font-medium text-sm">
                Data Source:
              </span>
              <div className="flex gap-2">
                <Button
                  type="button"
                  onClick={() => updateFilter('dataSource', 'vscode')}
                  variant={
                    filters.dataSource === 'vscode' ? 'default' : 'secondary'
                  }
                >
                  VSCode
                </Button>
                <Button
                  type="button"
                  onClick={() => updateFilter('dataSource', 'openvsx')}
                  variant={
                    filters.dataSource === 'openvsx' ? 'default' : 'secondary'
                  }
                >
                  OpenVSX
                </Button>
              </div>
            </div>

            {/* Search Input */}
            <div className="flex flex-col items-start gap-4 sm:flex-row sm:items-center">
              <Label
                htmlFor="search-input"
                className="min-w-fit font-medium text-sm"
              >
                Search:
              </Label>
              <Input
                id="search-input"
                type="text"
                value={filters.query}
                onChange={e => updateFilter('query', e.target.value)}
                placeholder="Search extensions..."
              />
            </div>

            {/* Additional Filters */}
            <div className="flex flex-col items-start gap-4 sm:flex-row sm:items-center">
              <Label
                htmlFor="version-input"
                className="min-w-fit font-medium text-sm"
              >
                VSCode Version:
              </Label>
              <Input
                id="version-input"
                type="text"
                value={filters.vscodeVersion}
                onChange={e => updateFilter('vscodeVersion', e.target.value)}
                placeholder="e.g., ^1.97.0 (optional)"
              />
            </div>

            <div className="flex items-center gap-2">
              <Checkbox
                id="prerelease"
                checked={filters.includePrerelease}
                onCheckedChange={checked => {
                  updateFilter('includePrerelease', checked === true);
                }}
              />
              <label htmlFor="prerelease" className="font-medium text-sm">
                Include prerelease versions
              </label>
            </div>
          </div>
        </div>

        {/* Results Section */}
        <div className="mx-auto max-w-6xl">
          {error ? (
            <div className="rounded-lg border border-destructive/20 bg-destructive/10 p-6">
              <div className="text-center text-destructive">
                <p className="font-medium">Error loading extensions</p>
                <p className="mt-1 text-sm">{error.message}</p>
              </div>
            </div>
          ) : (
            <VirtualExtensionList
              extensions={filteredExtensions}
              isLoading={isLoading}
            />
          )}
        </div>
      </div>
    </div>
  );
}

export default function SearchPage() {
  return (
    <QueryClientProvider client={queryClient}>
      <SearchContent />
      <Toaster />
    </QueryClientProvider>
  );
}
