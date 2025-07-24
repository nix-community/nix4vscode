import { useVirtualizer } from '@tanstack/react-virtual';
import { ChevronDown, ChevronRight } from 'lucide-react';
import { useEffect, useRef, useState } from 'react';
import { toast } from 'sonner';
import type { ExportedData, ExtensionItem } from '@/types/index';
import { Badge } from './ui/badge';

interface VirtualExtensionListProps {
  extensions: ExtensionItem[];
  isLoading?: boolean;
}

interface ExtensionItemComponentProps {
  extension: ExtensionItem;
  isExpanded: boolean;
  onToggleExpand: () => void;
}

function VersionRow({ version }: { version: ExportedData }) {
  return (
    <div className="flex items-center justify-between gap-4 border-border/50 border-t px-4 py-2 text-sm">
      <div className="flex min-w-0 flex-1 items-center gap-4">
        <Badge
          variant={'secondary'}
          className="rounded bg-muted px-2 py-1 font-mono text-xs"
        >
          {version.v}
        </Badge>
        <span className="truncate text-muted-foreground">{version.e}</span>
        <span className="text-muted-foreground text-xs">
          {version.p || 'Universal'}
        </span>
      </div>
      {version.r && (
        <span className="rounded bg-orange-100 px-1.5 py-0.5 text-orange-800 text-xs">
          Prerelease
        </span>
      )}
      <Badge
        variant="secondary"
        className="cursor-pointer font-mono"
        onClick={async () => {
          if (version.h) {
            await navigator.clipboard.writeText(version.h);
            toast.success('Hash copied to clipboard');
          }
        }}
      >
        {version.h.slice(0, 8)}...
      </Badge>
    </div>
  );
}

function ExtensionItemComponent({
  extension,
  isExpanded,
  onToggleExpand,
}: ExtensionItemComponentProps) {
  const latestVersion = extension.versions[0];
  const versionCount = extension.versions.length;

  return (
    <div className="border-border border-b transition-colors hover:bg-muted/50">
      <button
        type="button"
        onClick={versionCount > 1 ? onToggleExpand : undefined}
        className={`w-full p-4 text-left ${versionCount > 1 ? 'cursor-pointer' : 'cursor-default'}`}
      >
        <div className="flex items-start justify-between gap-4">
          <div className="min-w-0 flex-1">
            <div className="flex items-center gap-2">
              <h3 className="truncate font-medium text-foreground">
                {extension.name}
              </h3>
              {versionCount > 1 && (
                <div className="flex items-center gap-1 text-muted-foreground">
                  {isExpanded ? (
                    <ChevronDown className="h-4 w-4" />
                  ) : (
                    <ChevronRight className="h-4 w-4" />
                  )}
                  <span className="text-xs">{versionCount} versions</span>
                </div>
              )}
            </div>
            <div className="mt-1 text-muted-foreground text-sm">
              <span className="inline-flex items-center gap-2">
                <span>Latest: {latestVersion?.v}</span>
                {latestVersion?.r && (
                  <span className="rounded bg-orange-100 px-1.5 py-0.5 text-orange-800 text-xs">
                    Prerelease
                  </span>
                )}
              </span>
            </div>
            <div className="mt-1 text-muted-foreground text-xs">
              Engine: {latestVersion?.e} | Platform: {latestVersion?.p || 'Any'}
            </div>
          </div>
          <div className="flex flex-col items-end gap-1">
            <Badge
              variant="secondary"
              className="cursor-pointer font-mono"
              onClick={async e => {
                e.stopPropagation();
                if (latestVersion.h) {
                  await navigator.clipboard.writeText(latestVersion.h);
                  toast.success('Hash copied to clipboard');
                }
              }}
            >
              {latestVersion?.h.slice(0, 8)}...
            </Badge>
          </div>
        </div>
      </button>

      {isExpanded && versionCount > 1 && (
        <div className="bg-muted/20">
          <div className="border-border/50 border-t px-4 py-2 font-medium text-muted-foreground text-xs">
            All Versions
          </div>
          {extension.versions.map((version, index) => (
            <VersionRow key={`${version.v}-${index}`} version={version} />
          ))}
        </div>
      )}
    </div>
  );
}

export default function VirtualExtensionList({
  extensions,
  isLoading = false,
}: VirtualExtensionListProps) {
  const parentRef = useRef<HTMLDivElement>(null);
  const [expandedItems, setExpandedItems] = useState<Set<string>>(new Set());

  const toggleExpand = (extensionName: string) => {
    setExpandedItems(prev => {
      const newSet = new Set(prev);
      if (newSet.has(extensionName)) {
        newSet.delete(extensionName);
      } else {
        newSet.add(extensionName);
      }
      return newSet;
    });
  };

  const virtualizer = useVirtualizer({
    count: extensions.length,
    getScrollElement: () => parentRef.current,
    estimateSize: index => {
      const extension = extensions[index];
      const isExpanded = expandedItems.has(extension.name);
      const baseHeight = 120;
      if (isExpanded && extension.versions.length > 1) {
        // Base height + header for "All Versions" + each version row
        const expandedHeight = 24 + extension.versions.length * 45;
        return baseHeight + expandedHeight;
      }
      return baseHeight;
    },
    overscan: 5,
  });

  // Force remeasure when expanded items change
  useEffect(() => {
    virtualizer.measure();
  }, [expandedItems, virtualizer]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="flex items-center gap-2 text-muted-foreground">
          <div className="h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent" />
          Loading extensions...
        </div>
      </div>
    );
  }

  if (extensions.length === 0) {
    return (
      <div className="py-12 text-center text-muted-foreground">
        <p>No extensions found matching your criteria.</p>
        <p className="mt-1 text-sm">Try adjusting your search or filters.</p>
      </div>
    );
  }

  return (
    <div className="overflow-hidden rounded-lg border">
      <div className="border-b bg-muted/30 px-4 py-2">
        <span className="font-medium text-sm">
          {extensions.length} extension{extensions.length !== 1 ? 's' : ''}{' '}
          found
        </span>
      </div>

      <div ref={parentRef} className="h-[600px] overflow-auto">
        <div
          style={{
            height: `${virtualizer.getTotalSize()}px`,
            width: '100%',
            position: 'relative',
          }}
        >
          {virtualizer.getVirtualItems().map(virtualItem => (
            <div
              key={virtualItem.key}
              style={{
                position: 'absolute',
                top: 0,
                left: 0,
                width: '100%',
                height: `${virtualItem.size}px`,
                transform: `translateY(${virtualItem.start}px)`,
              }}
            >
              <ExtensionItemComponent
                extension={extensions[virtualItem.index]}
                isExpanded={expandedItems.has(
                  extensions[virtualItem.index].name,
                )}
                onToggleExpand={() =>
                  toggleExpand(extensions[virtualItem.index].name)
                }
              />
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
