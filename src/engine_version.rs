use anyhow::anyhow;
use lazy_regex::regex;

#[derive(Debug, Clone)]
pub struct RequiredVersion {
    major: Option<u64>,
    minor: Option<u64>,
    patch: Option<u64>,
    is_upper: bool,
    is_preview: bool,
}

impl RequiredVersion {
    fn is_any(&self) -> bool {
        self.major.is_none() && self.minor.is_none() && self.patch.is_none()
    }
    pub fn new(version: &str) -> anyhow::Result<Self> {
        if version.trim() == "*" {
            return Ok(Self {
                major: None,
                minor: None,
                patch: None,
                is_upper: false,
                is_preview: false,
            });
        }
        let v = regex!(r#"(\^)?(\d+).(\d+).(\d*x?)(-.*)?"#);
        let v = v
            .captures(version)
            .ok_or(anyhow!(format!("bad version: {version}")))?;

        Ok(Self {
            is_upper: &v[4] == "x" || v.get(1).is_some(),
            major: Some(v[2].parse().unwrap()),
            minor: Some(v[3].parse().unwrap()),
            patch: match &v[4] {
                "x" => None,
                var => Some(var.parse().unwrap()),
            },
            is_preview: v.get(5).is_some(),
        })
    }

    pub fn is_equal(&self, rhs: &Self) -> bool {
        self.major == rhs.major && self.minor == rhs.minor && self.patch == rhs.patch
    }

    /// self is version req, rhs is version
    pub fn is_matched(&self, rhs: &Self) -> bool {
        if self.is_any() {
            return true;
        }
        if self.is_upper {
            if self.major < rhs.major {
                return true;
            }
            if self.major > rhs.major {
                return false;
            }

            if self.minor < rhs.minor {
                return true;
            }

            if self.minor > rhs.minor {
                return false;
            }
            if self.patch.is_none() {
                return true;
            }

            if self.patch <= rhs.patch {
                return true;
            }

            return false;
        }

        self.major == rhs.major
            && self.minor == rhs.minor
            && (self.patch.is_none() || self.patch == rhs.patch)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_visual_studio_code_compatibility() {
        let v = RequiredVersion::new("1.8.1").unwrap();

        let vs = [
            ("1.8.1", true),
            ("^0.1.1", true),
            ("^1.2.1", true),
            ("1.81.1-inside", false),
            ("1.0.1-inside", false),
            ("^1.81.1-inside", false),
            ("^1.0.1-inside", true),
            ("1.12.1", false),
            ("^1.12.1", false),
            ("*", true),
            ("0.10.x", true),
            ("^0.10.x", true),
            ("1.8.x", true),
        ];

        for (i, k) in vs {
            let a = RequiredVersion::new(i).unwrap();
            assert!(a.is_matched(&v) == k);
        }
    }
}
