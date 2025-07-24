import type { ExportedData } from '../../../crates/exporter/bindings/ExportedData';

export type { ExportedData };

export type ExtensionData = Record<string, ExportedData[]>;

export type DataSource = 'vscode' | 'openvsx';

export interface SearchFilters {
  query: string;
  dataSource: DataSource;
  vscodeVersion?: string;
  includePrerelease: boolean;
}

export interface ExtensionItem {
  name: string;
  versions: ExportedData[];
}
