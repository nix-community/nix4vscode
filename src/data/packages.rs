mod default;
mod ignore;

use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    pub engines: Engines,
}

impl PackageJson {
    pub fn is_compate_with(&self, target_version: &semver::Version) -> bool {
        let required_ver = semver::VersionReq::from_str(&self.engines.vscode).unwrap();
        required_ver.matches(target_version)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Engines {
    pub vscode: String,
}
