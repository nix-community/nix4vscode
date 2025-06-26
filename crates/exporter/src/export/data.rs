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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p: Option<String>,
    /// hash
    pub h: String,
    /// is_prerelease
    #[serde(skip_serializing_if = "is_false")]
    pub r: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u: Option<String>,
}

impl From<Marketplace> for ExportedData {
    fn from(value: Marketplace) -> Self {
        let Marketplace {
            name: _,
            publisher: _,
            version,
            engine,
            platform,
            is_prerelease,
            hash,
            url,
        } = value;
        Self {
            v: version,
            e: engine,
            p: if platform == "universal" {
                None
            } else {
                Some(platform)
            },
            h: hash.unwrap(),
            r: is_prerelease,
            u: url,
        }
    }
}

fn is_false(v: &bool) -> bool {
    !v
}
