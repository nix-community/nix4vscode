// https://github.com/microsoft/vscode/blob/747b90cfcd486731a1295563423b1ee66dd9b613/src/vs/platform/extensions/common/extensionValidator.ts
#![allow(dead_code)]
#[cfg(test)]
mod test;
use std::str::FromStr;

use anyhow::anyhow;
use chrono::{FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Utc};
use lazy_regex::regex;

#[derive(Default, Eq, PartialEq, Debug)]
pub struct IParsedVersion {
    pub has_caret: bool,
    pub has_greater_equals: bool,
    pub major_base: i32,
    pub major_must_equal: bool,
    pub minor_base: i32,
    pub minor_must_equal: bool,
    pub patch_base: i32,
    pub patch_must_equal: bool,
    pub pre_release: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct INormalizedVersion {
    major_base: i32,
    major_must_equal: bool,
    minor_base: i32,
    minor_must_equal: bool,
    patch_base: i32,
    patch_must_equal: bool,
    not_before: i64, // milliseconds timestamp, or 0
    is_minimum: bool,
}

static VERSION_REGEXP: &lazy_regex::Lazy<lazy_regex::Regex> =
    regex!(r#"^(\^|>=)?((\d+)|x)\.((\d+)|x)\.((\d+)|x)(\-.*)?$"#);
static NOT_BEFORE_REGEXP: &lazy_regex::Lazy<lazy_regex::Regex> =
    regex!(r#"^-(\d{4})(\d{2})(\d{2})$"#);

pub fn is_valid_version_str(version: &str) -> bool {
    let version = version.trim();
    version == "*" || VERSION_REGEXP.is_match(version)
}

pub fn parse_version(version: &str) -> Option<IParsedVersion> {
    if !is_valid_version_str(version) {
        return None;
    }

    let version = version.trim();

    if version == "*" {
        return Some(IParsedVersion {
            has_caret: false,
            has_greater_equals: false,
            major_base: 0,
            major_must_equal: false,
            minor_base: 0,
            minor_must_equal: false,
            patch_base: 0,
            patch_must_equal: false,
            pre_release: None,
        });
    }

    let m = VERSION_REGEXP.captures(version);
    let m = m?;

    macro_rules! xxx {
        ($idx:expr, $x:expr, $exp:expr) => {
            if let Some(m) = $idx {
                $x = m.as_str() == $exp;
            }
        };
    }
    macro_rules! yyy {
        ($idx:expr, $x:expr, $exp:expr, $default:expr, $else:expr) => {
            if let Some(m) = $idx {
                $x = if m.as_str() == $exp { $default } else { $else };
            };
        };
        ($idx:expr, $x:expr, $exp:expr, $default:expr) => {
            if let Some(m) = $idx {
                $x = if m.as_str() == $exp {
                    $default
                } else {
                    m.as_str().parse().unwrap()
                };
            };
        };
    }
    let mut res = IParsedVersion::default();
    xxx!(m.get(1), res.has_caret, "^");
    xxx!(m.get(1), res.has_greater_equals, ">=");
    yyy!(m.get(2), res.major_base, "x", 0);
    yyy!(m.get(2), res.major_must_equal, "x", false, true);
    yyy!(m.get(4), res.minor_base, "x", 0);
    yyy!(m.get(4), res.minor_must_equal, "x", false, true);
    yyy!(m.get(6), res.patch_base, "x", 0);
    yyy!(m.get(6), res.patch_must_equal, "x", false, true);

    if let Some(m) = m.get(8) {
        res.pre_release = Some(m.as_str().to_string())
    }

    Some(res)
}

pub fn normalize_version(version: IParsedVersion) -> INormalizedVersion {
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
    if let Some(pre_release) = &version.pre_release {
        let m = NOT_BEFORE_REGEXP.captures(pre_release);
        if let Some(m) = m {
            let year = m.get(1).map(|item| item.as_str()).unwrap_or("0");
            let month = m.get(2).map(|item| item.as_str()).unwrap_or("0");
            let day = m.get(3).map(|item| item.as_str()).unwrap_or("0");

            not_before = NaiveDate::from_ymd_opt(
                year.parse().unwrap_or_default(),
                month.parse::<u32>().unwrap_or(1) - 1,
                day.parse().unwrap_or_default(),
            )
            .and_then(|date| {
                let datetime = NaiveDateTime::new(date, Default::default());

                FixedOffset::east_opt(0).and_then(|of| {
                    let i = of
                        .from_local_datetime(&datetime)
                        .map(|dwz| dwz.naive_utc())
                        .map(|dwz| Utc.from_utc_datetime(&dwz).timestamp_millis());
                    match i {
                        chrono::offset::LocalResult::Single(v) => Some(v),
                        _ => None,
                    }
                })
            })
            .unwrap_or_default();
        }
    }

    INormalizedVersion {
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

impl FromStr for INormalizedVersion {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(normalize_version(
            parse_version(s).ok_or(anyhow!("bad version"))?,
        ))
    }
}
pub fn is_valid_version(
    version: INormalizedVersion,
    product_ts: Option<i64>,
    desired_version: INormalizedVersion,
) -> bool {
    // let productTs: number | undefined;
    // if (_inputDate instanceof Date) {
    // 	productTs = _inputDate.getTime();
    // } else if (typeof _inputDate === 'string') {
    // 	productTs = new Date(_inputDate).getTime();
    // }
    //

    let major_base = version.major_base;
    let minor_base = version.minor_base;
    let patch_base = version.patch_base;

    let mut desired_major_base = desired_version.major_base;
    let mut desired_minor_base = desired_version.minor_base;
    let mut desired_patch_base = desired_version.patch_base;
    let desired_not_before = desired_version.not_before;

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

        if let Some(ts) = product_ts {
            if ts < desired_not_before {
                return false;
            }
        }

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

    if let Some(ts) = product_ts {
        if ts < desired_not_before {
            return false;
        }
    }

    true
}

/// `current_version`: code version.
/// `requested_version`: extension requested version.
pub fn is_version_valid(
    current_version: &str,
    date: Option<i64>,
    requested_version: &str,
) -> anyhow::Result<bool> {
    let desired_version: INormalizedVersion = requested_version.parse()?;

    // enforce that a breaking API version is specified.
    // for 0.X.Y, that means up to 0.X must be specified
    // otherwise for Z.X.Y, that means Z must be specified
    if desired_version.major_base == 0 {
        // force that major and minor must be specific
        if !desired_version.major_must_equal || !desired_version.minor_must_equal {
            return Err(anyhow!( "Version specified in `engines.vscode` ({0}) is not specific enough. For vscode versions before 1.0.0, please define at a minimum the major and minor desired version. E.g. ^0.10.0, 0.10.x, 0.11.0, etc.", requested_version));
        }
    } else {
        // force that major must be specific
        if !desired_version.major_must_equal {
            return Err(anyhow!("Version specified in `engines.vscode` ({0}) is not specific enough. For vscode versions after 1.0.0, please define at a minimum the major desired version. E.g. ^1.10.0, 1.10.x, 1.x.x, 2.x.x, etc.", requested_version));
        }
    }

    if !is_valid_version(current_version.parse()?, date, desired_version) {
        return Err(anyhow!(
            "Extension is not compatible with Code {0}. Extension requires: {1}.",
            current_version,
            requested_version
        ));
    }

    Ok(true)
}
