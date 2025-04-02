import { ExportedData } from '../crates/exporter/bindings/ExportedData.ts';

interface Marketplace extends ExportedData {
  /// Url
  u?: string;
}

export interface MarketplaceJson {
  [key: string]: Marketplace[];
}

export interface NameVersion {
  [key: string]: string;
}
