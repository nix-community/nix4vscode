use anyhow::anyhow;
use chrono::{NaiveDate, NaiveDateTime};
use lazy_regex::regex;
use tracing::debug;

macro_rules! texpr {
    ($ex1:expr => $ex2:expr , $ex3:expr) => {
        if $ex1 {
            $ex2
        } else {
            $ex3
        }
    };
}

#[derive(Debug, Clone, Default)]
struct IParsedVersion {
    has_caret: bool,
    has_greater_equals: bool,
    major_base: u64,
    major_must_equal: bool,
    minor_base: u64,
    minor_must_equal: bool,
    patch_base: u64,
    patch_must_equal: bool,
    pre_release: Option<String>,
}

impl IParsedVersion {
    fn new(version: &str) -> anyhow::Result<Self> {
        let version = version.trim();
        if version == "*" {
            return Ok(Default::default());
        }

        let m = regex!(r#"^(\^|>=)?((\d+)|x)\.((\d+)|x)\.((\d+)|x)(\-.*)?$"#)
            .captures(version)
            .ok_or(anyhow!(format!("bad version: {version}")))?;

        Ok(Self {
            has_caret: m.get(1).map(|item| item.as_str()) == Some("^"),
            has_greater_equals: m.get(1).map(|item| item.as_str()) == Some(">="),
            major_base: texpr!(&m[2] == "x" => 0, m[2].parse()?),
            major_must_equal: texpr!(&m[2] == "x" => false, true),
            minor_base: texpr!(&m[4] == "x" => 0, m[4].parse()?),
            minor_must_equal: texpr!(&m[4] == "x" => false, true),
            patch_base: texpr!(&m[6] == "x" => 0, m[6].parse()?),
            patch_must_equal: texpr!(&m[6] == "x" => false, true),
            pre_release: m.get(8).map(|item| item.as_str().to_string()),
        })
    }
}

#[derive(Debug, Clone)]
struct INormalizedVersion {
    major_base: u64,
    major_must_equal: bool,
    minor_base: u64,
    minor_must_equal: bool,
    patch_base: u64,
    patch_must_equal: bool,
    not_before: i64, // milliseconds timestamp, or 0
    is_minimum: bool,
}

impl From<IParsedVersion> for INormalizedVersion {
    fn from(version: IParsedVersion) -> Self {
        let major_base = version.major_base;
        let major_must_equal = version.major_must_equal;
        let minor_base = version.minor_base;
        let mut minor_must_equal = version.minor_must_equal;
        let patch_base = version.patch_base;
        let mut patch_must_equal = version.patch_must_equal;

        if version.has_caret {
            if major_base == 0 {
                patch_must_equal = false;
            } else {
                minor_must_equal = false;
                patch_must_equal = false;
            }
        }

        let mut not_before = 0;
        if let Some(pre_release) = version.pre_release {
            if let Some(m) = regex!(r#"/^-(\d{4})(\d{2})(\d{2})$/"#).captures(&pre_release) {
                let year = &m[1];
                let month = &m[2];
                let day = &m[3];

                let a = (|| -> anyhow::Result<i64> {
                    let a: NaiveDateTime =
                        NaiveDate::from_ymd_opt(year.parse()?, month.parse()?, day.parse()?)
                            .ok_or(anyhow!(format!("bad version: {year}.{month}.{day}")))?
                            .into();
                    Ok(a.and_utc().timestamp_millis())
                })();

                match a {
                    Ok(v) => not_before = v,
                    Err(err) => {
                        debug!(?err);
                    }
                }
            }
        }

        Self {
            major_base,
            major_must_equal,
            minor_base,
            minor_must_equal,
            patch_base,
            patch_must_equal,
            is_minimum: version.has_greater_equals,
            not_before,
        }
    }
}

pub fn is_version_valid(
    code_version: &str,
    // date: DateTime<Utc>,
    requested_version: &str,
) -> bool {
    if requested_version.trim() == "*" {
        return true;
    }
    let desired_version: INormalizedVersion = match IParsedVersion::new(requested_version) {
        Ok(v) => v.into(),
        Err(err) => {
            debug!(?err);
            return false;
        }
    };

    if desired_version.major_base == 0 {
        if !desired_version.major_must_equal || !desired_version.minor_must_equal {
            return false;
        }
    } else if !desired_version.major_must_equal {
        return false;
    }

    let current_version: INormalizedVersion = match IParsedVersion::new(code_version) {
        Ok(v) => v.into(),
        Err(err) => {
            debug!(?err);
            return false;
        }
    };
    if !is_valid_version(current_version, desired_version) {
        return false;
    }

    true
}

fn is_valid_version(
    version: INormalizedVersion,
    // date: DateTime<Utc>,
    desired_version: INormalizedVersion,
) -> bool {
    // let product_ts = date.timestamp_millis();

    let major_base = version.major_base;
    let minor_base = version.minor_base;
    let patch_base = version.patch_base;

    let mut desired_major_base = desired_version.major_base;
    let mut desired_minor_base = desired_version.minor_base;
    let mut desired_patch_base = desired_version.patch_base;
    let _desired_not_before = desired_version.not_before;

    let mut major_must_equal = desired_version.major_must_equal;
    let mut minor_must_equal = desired_version.minor_must_equal;
    let mut patch_must_equal = desired_version.patch_must_equal;

    if desired_version.is_minimum {
        if major_base > desired_major_base {
            return true;
        }

        if major_base < desired_major_base {
            return false;
        }

        if minor_base > desired_minor_base {
            return true;
        }

        if minor_base < desired_minor_base {
            return false;
        }

        // if product_ts < desired_not_before {
        //     return false;
        // }

        return patch_base >= desired_patch_base;
    }

    // Anything < 1.0.0 is compatible with >= 1.0.0, except exact matches
    if major_base == 1
        && desired_major_base == 0
        && (!major_must_equal || !minor_must_equal || !patch_must_equal)
    {
        desired_major_base = 1;
        desired_minor_base = 0;
        desired_patch_base = 0;
        major_must_equal = true;
        minor_must_equal = false;
        patch_must_equal = false;
    }

    if major_base < desired_major_base {
        // smaller major version
        return false;
    }

    if major_base > desired_major_base {
        // higher major version
        return !major_must_equal;
    }

    // at this point, majorBase are equal

    if minor_base < desired_minor_base {
        // smaller minor version
        return false;
    }

    if minor_base > desired_minor_base {
        // higher minor version
        return !minor_must_equal;
    }

    // at this point, minorBase are equal

    if patch_base < desired_patch_base {
        // smaller patch version
        return false;
    }

    if patch_base > desired_patch_base {
        // higher patch version
        return !patch_must_equal;
    }

    // at this point, patchBase are equal

    // if product_ts < desired_not_before {
    //     return false;
    // }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_version_valid() {
        let v = "1.8.1";

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
            assert_eq!(k, is_version_valid(v, i));
        }
    }
}
