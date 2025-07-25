import { QueryClientProvider } from '@tanstack/react-query';
import { Search } from 'lucide-react';
import { useId, useState } from 'react';
import { VscVscode } from 'react-icons/vsc';
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import { Checkbox } from '@/components/ui/checkbox';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Toaster } from '@/components/ui/sonner';
import { useSearch } from '@/hooks/useSearch';
import { queryClient } from '@/lib/query-client';
import { useExtensionData } from '@/services/extension-data';
import type { SearchFilters } from '@/types/index';
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
  const search_input = useId();
  const version_input = useId();
  const prerelease = useId();

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

        <Card className="mx-auto mb-8 max-w-4xl">
          <CardContent className="flex flex-col gap-4">
            <div className="flex flex-col items-start gap-4 sm:flex-row sm:items-center">
              <span>Data Source: </span>
              <div className="flex gap-4">
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

            <div className="flex flex-row items-center gap-4">
              <Label
                htmlFor={search_input}
                className="min-w-fit font-medium text-sm"
              >
                <Search className="mt-1 h-6 w-6" />
              </Label>
              <Input
                id={search_input}
                type="text"
                value={filters.query}
                onChange={e => updateFilter('query', e.target.value)}
                placeholder="Search extensions..."
              />
            </div>

            <div className="flex flex-row items-center gap-4">
              <Label
                htmlFor={version_input}
                className="min-w-fit font-medium text-sm"
              >
                <VscVscode className="mt-1 h-6 w-6" />
              </Label>
              <Input
                id={version_input}
                type="text"
                value={filters.vscodeVersion}
                onChange={e => updateFilter('vscodeVersion', e.target.value)}
                placeholder="e.g., ^1.97.0 (optional)"
              />
            </div>

            <div className="flex items-center gap-2">
              <Checkbox
                id={prerelease}
                checked={filters.includePrerelease}
                onCheckedChange={checked => {
                  updateFilter('includePrerelease', checked === true);
                }}
              />
              <label htmlFor={prerelease} className="font-medium text-sm">
                Include prerelease versions
              </label>
            </div>
          </CardContent>
        </Card>

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
