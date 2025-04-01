import { ExportedData } from '../crates/exporter/bindings/ExportedData.ts';

interface Marketplace extends ExportedData {
  /// Url
  u?: string;
}

interface MarketplaceJson {
  [key: string]: Marketplace[];
}

interface NameVersion {
  [key: string]: string;
}
