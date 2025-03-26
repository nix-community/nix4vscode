interface Marketplace {
  n: string;
  v: string;
  e: string;
  platform?: string;
  u: string;
  h: string;
}

interface MarketplaceJson {
  [key: string]: Marketplace[];
}

interface NameVersion {
  [key: string]: string;
}
