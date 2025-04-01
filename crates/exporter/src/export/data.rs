use serde::Deserialize;
use serde::Serialize;

use super::Marketplace;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, ts_rs::TS)]
#[ts(export)]
pub(crate) struct ExportedData {
    /// version
    pub v: String,
    /// engine
    pub e: String,
    /// platform
    #[serde(skip_serializing_if = "is_universal")]
    pub p: String,
    /// hash
    pub h: String,
    /// is_prerelease
    #[serde(skip_serializing_if = "is_false")]
    pub r: bool,
}

impl From<Marketplace> for ExportedData {
    fn from(value: Marketplace) -> Self {
        let Marketplace {
            name: _,
            publisher: _,
            version,
            engine,
            platform,
            assert_url: _,
            is_prerelease,
            hash,
        } = value;
        Self {
            v: version,
            e: engine,
            p: platform,
            h: hash.unwrap(),
            r: is_prerelease,
        }
    }
}

fn is_universal(name: &str) -> bool {
    name == "universal"
}

fn is_false(v: &bool) -> bool {
    !v
}
