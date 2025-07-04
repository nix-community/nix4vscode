import { ExportedData } from '../../crates/exporter/bindings/ExportedData';

interface Marketplace extends ExportedData {}

export interface MarketplaceJson {
  [key: string]: Marketplace[];
}

export interface NameVersion {
  [key: string]: string;
}
