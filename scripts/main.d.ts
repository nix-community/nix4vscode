interface Marketplace {
  n: string;
  v: string;
  e: string;
  platform?: string;
  u: string;
  h: string;
  /// is_prelease
  r?: boolean;
}

interface MarketplaceJson {
  [key: string]: Marketplace[];
}

interface NameVersion {
  [key: string]: string;
}
