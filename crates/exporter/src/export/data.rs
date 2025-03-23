use serde::Deserialize;
use serde::Serialize;

use super::Marketplace;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ExportedData {
    pub version: String,
    pub engine: String,
    #[serde(skip_serializing_if = "is_universal")]
    pub platform: String,
    pub url: String,
    pub hash: Option<String>,
}

impl From<Marketplace> for ExportedData {
    fn from(value: Marketplace) -> Self {
        let Marketplace {
            name: _,
            publisher: _,
            version,
            engine,
            platform,
            assert_url,
            is_prerelease: _,
            hash,
        } = value;
        Self {
            version,
            engine,
            platform,
            url: assert_url,
            hash,
        }
    }
}

fn is_universal(name: &str) -> bool {
    name == "universal"
}
