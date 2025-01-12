use code_api::config::SystemContext;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AssetUrlContext {
    system: SystemContext,
    extension: ExtensionContextInner,
}

impl AssetUrlContext {
    pub fn new(system: SystemContext, version: String) -> Self {
        Self {
            system,
            extension: ExtensionContextInner { version },
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct ExtensionContextInner {
    version: String,
}
