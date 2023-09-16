use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct SystemContext {
    arch: String,
    ostype: String,
}

impl Default for SystemContext {
    fn default() -> Self {
        Self {
            arch: std::env::consts::ARCH.into(),
            ostype: std::env::consts::OS.into(),
        }
    }
}
