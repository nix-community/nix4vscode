interface Marketplace {
  /**
   * version
   */
  v: string;
  /**
   * engine
   */
  e: string;
  /**
   * platform
   */
  p?: string;
  /**
   * assert_url
   */
  u: string;
  /**
   * hash
   */
  h: string;
  /**
   * is_prelease
   */
  r?: boolean;
}

interface MarketplaceJson {
  [key: string]: Marketplace[];
}

interface NameVersion {
  [key: string]: string;
}
