use serde::Deserialize;
use serde::Serialize;

use super::Marketplace;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ExportedData {
    pub v: String,
    pub e: String,
    #[serde(skip_serializing_if = "is_universal")]
    pub p: String,
    pub u: String,
    pub h: String,
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
            assert_url,
            is_prerelease,
            hash,
        } = value;
        Self {
            v: version,
            e: engine,
            p: platform,
            u: assert_url,
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
