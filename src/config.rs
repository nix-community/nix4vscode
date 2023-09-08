use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub vscode_version: String,
    pub extensions: Vec<Extension>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Extension {
    pub publisher_name: String,
    pub extension_name: String,
}
