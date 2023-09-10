use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AssetUrlContext {
    extension: ExtensionContextInner,
}

impl AssetUrlContext {
    pub fn new(version: String) -> Self {
        Self {
            extension: ExtensionContextInner { version },
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct ExtensionContextInner {
    version: String,
}
