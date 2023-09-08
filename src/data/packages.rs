mod default;
mod ignore;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    pub engines: Engines,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Engines {
    pub vscode: String,
}
