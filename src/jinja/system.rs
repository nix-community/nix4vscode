use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemContext<'a> {
    arch: &'a str,
    ostype: &'a str,
}

impl<'a> Default for SystemContext<'a> {
    fn default() -> Self {
        Self {
            arch: std::env::consts::ARCH,
            ostype: std::env::consts::OS,
        }
    }
}
