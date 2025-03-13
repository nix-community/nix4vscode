use chrono::DateTime;

// https://github.com/microsoft/vscode/blob/430a653d1b802e3073892814993e9014f1289d8b/src/vs/platform/extensions/test/common/extensionValidator.test.ts
use super::*;

#[test]
fn test_is_valid_version_str() {
    assert!(is_valid_version_str("0.10.0-dev"));
    assert!(is_valid_version_str("0.10.0"));
    assert!(is_valid_version_str("0.10.1"));
    assert!(is_valid_version_str("0.10.100"));
    assert!(is_valid_version_str("0.11.0"));

    assert!(is_valid_version_str("x.x.x"));
    assert!(is_valid_version_str("0.x.x"));
    assert!(is_valid_version_str("0.10.0"));
    assert!(is_valid_version_str("0.10.x"));
    assert!(is_valid_version_str("^0.10.0"));
    assert!(is_valid_version_str("*"));

    assert!(!is_valid_version_str("0.x.x.x"));
    assert!(!is_valid_version_str("0.10"));
    assert!(!is_valid_version_str("0.10."));
}

#[test]
fn test_parse_version() {
    #[allow(clippy::too_many_arguments)]
    fn assert_parse_version(
        version: &str,
        has_caret: bool,
        has_greater_equals: bool,
        major_base: i32,
        major_must_equal: bool,
        minor_base: i32,
        minor_must_equal: bool,
        patch_base: i32,
        patch_must_equal: bool,
        pre_release: Option<String>,
    ) {
        let actual = parse_version(version);
        let expected = IParsedVersion {
            has_caret,
            has_greater_equals,
            major_base,
            major_must_equal,
            minor_base,
            minor_must_equal,
            patch_base,
            patch_must_equal,
            pre_release,
        };

        assert_eq!(actual, Some(expected), "parserVersion for {version}");
    }

    assert_parse_version(
        "0.10.0-dev",
        false,
        false,
        0,
        true,
        10,
        true,
        0,
        true,
        Some("-dev".into()),
    );
    assert_parse_version("0.10.0", false, false, 0, true, 10, true, 0, true, None);
    assert_parse_version("0.10.1", false, false, 0, true, 10, true, 1, true, None);
    assert_parse_version("0.10.100", false, false, 0, true, 10, true, 100, true, None);
    assert_parse_version("0.11.0", false, false, 0, true, 11, true, 0, true, None);

    assert_parse_version("x.x.x", false, false, 0, false, 0, false, 0, false, None);
    assert_parse_version("0.x.x", false, false, 0, true, 0, false, 0, false, None);
    assert_parse_version("0.10.x", false, false, 0, true, 10, true, 0, false, None);
    assert_parse_version("^0.10.0", true, false, 0, true, 10, true, 0, true, None);
    assert_parse_version("^0.10.2", true, false, 0, true, 10, true, 2, true, None);
    assert_parse_version("^1.10.2", true, false, 1, true, 10, true, 2, true, None);
    assert_parse_version("*", false, false, 0, false, 0, false, 0, false, None);

    assert_parse_version(">=0.0.1", false, true, 0, true, 0, true, 1, true, None);
    assert_parse_version(">=2.4.3", false, true, 2, true, 4, true, 3, true, None);
}

#[test]
fn test_normalize_version() {
    #[allow(clippy::too_many_arguments)]
    fn assert_normalize_version(
        version: &str,
        major_base: i32,
        major_must_equal: bool,
        minor_base: i32,
        minor_must_equal: bool,
        patch_base: i32,
        patch_must_equal: bool,
        is_minimum: bool,
        not_before: i64,
    ) {
        let actual = normalize_version(parse_version(version).unwrap());
        let expected = INormalizedVersion {
            major_base,
            major_must_equal,
            minor_base,
            minor_must_equal,
            patch_base,
            patch_must_equal,
            not_before,
            is_minimum,
        };
        assert_eq!(actual, expected, "parseVersion for {version}");
    }

    assert_normalize_version("0.10.0-dev", 0, true, 10, true, 0, true, false, 0);
    assert_normalize_version("0.10.0-222222222", 0, true, 10, true, 0, true, false, 0);
    // assertNormalizeVersion("0.10.0-20210511", 0, true, 10, true, 0, true, false, new Date('2021-05-11T00:00:00Z').getTime());

    assert_normalize_version("0.10.0", 0, true, 10, true, 0, true, false, 0);
    assert_normalize_version("0.10.1", 0, true, 10, true, 1, true, false, 0);
    assert_normalize_version("0.10.100", 0, true, 10, true, 100, true, false, 0);
    assert_normalize_version("0.11.0", 0, true, 11, true, 0, true, false, 0);

    assert_normalize_version("x.x.x", 0, false, 0, false, 0, false, false, 0);
    assert_normalize_version("0.x.x", 0, true, 0, false, 0, false, false, 0);
    assert_normalize_version("0.10.x", 0, true, 10, true, 0, false, false, 0);
    assert_normalize_version("^0.10.0", 0, true, 10, true, 0, false, false, 0);
    assert_normalize_version("^0.10.2", 0, true, 10, true, 2, false, false, 0);
    assert_normalize_version("^1.10.2", 1, true, 10, false, 2, false, false, 0);
    assert_normalize_version("*", 0, false, 0, false, 0, false, false, 0);

    assert_normalize_version(">=0.0.1", 0, true, 0, true, 1, true, true, 0);
    assert_normalize_version(">=2.4.3", 2, true, 4, true, 3, true, true, 0);
    assert_normalize_version(">=2.4.3", 2, true, 4, true, 3, true, true, 0);
}

#[test]
fn test_is_valid_version() {
    fn test_is_valid_version(version: &str, desired_version: &str, expected_result: bool) {
        let product_version = "2021-05-11T21:54:30.577Z";
        let product_version: DateTime<Utc> = product_version.parse().unwrap();
        let product_version = product_version.timestamp_millis();
        let actual = is_valid_version(
            version.parse().unwrap(),
            Some(product_version),
            desired_version.parse().unwrap(),
        );
        assert_eq!(actual, expected_result);
    }

    test_is_valid_version("0.10.0-dev", "x.x.x", true);
    test_is_valid_version("0.10.0-dev", "0.x.x", true);
    test_is_valid_version("0.10.0-dev", "0.10.0", true);
    test_is_valid_version("0.10.0-dev", "0.10.2", false);
    test_is_valid_version("0.10.0-dev", "^0.10.2", false);
    test_is_valid_version("0.10.0-dev", "0.10.x", true);
    test_is_valid_version("0.10.0-dev", "^0.10.0", true);
    test_is_valid_version("0.10.0-dev", "*", true);
    test_is_valid_version("0.10.0-dev", ">=0.0.1", true);
    test_is_valid_version("0.10.0-dev", ">=0.0.10", true);
    test_is_valid_version("0.10.0-dev", ">=0.10.0", true);
    test_is_valid_version("0.10.0-dev", ">=0.10.1", false);
    test_is_valid_version("0.10.0-dev", ">=1.0.0", false);

    test_is_valid_version("0.10.0", "x.x.x", true);
    test_is_valid_version("0.10.0", "0.x.x", true);
    test_is_valid_version("0.10.0", "0.10.0", true);
    test_is_valid_version("0.10.0", "0.10.2", false);
    test_is_valid_version("0.10.0", "^0.10.2", false);
    test_is_valid_version("0.10.0", "0.10.x", true);
    test_is_valid_version("0.10.0", "^0.10.0", true);
    test_is_valid_version("0.10.0", "*", true);

    test_is_valid_version("0.10.1", "x.x.x", true);
    test_is_valid_version("0.10.1", "0.x.x", true);
    test_is_valid_version("0.10.1", "0.10.0", false);
    test_is_valid_version("0.10.1", "0.10.2", false);
    test_is_valid_version("0.10.1", "^0.10.2", false);
    test_is_valid_version("0.10.1", "0.10.x", true);
    test_is_valid_version("0.10.1", "^0.10.0", true);
    test_is_valid_version("0.10.1", "*", true);

    test_is_valid_version("0.10.100", "x.x.x", true);
    test_is_valid_version("0.10.100", "0.x.x", true);
    test_is_valid_version("0.10.100", "0.10.0", false);
    test_is_valid_version("0.10.100", "0.10.2", false);
    test_is_valid_version("0.10.100", "^0.10.2", true);
    test_is_valid_version("0.10.100", "0.10.x", true);
    test_is_valid_version("0.10.100", "^0.10.0", true);
    test_is_valid_version("0.10.100", "*", true);

    test_is_valid_version("0.11.0", "x.x.x", true);
    test_is_valid_version("0.11.0", "0.x.x", true);
    test_is_valid_version("0.11.0", "0.10.0", false);
    test_is_valid_version("0.11.0", "0.10.2", false);
    test_is_valid_version("0.11.0", "^0.10.2", false);
    test_is_valid_version("0.11.0", "0.10.x", false);
    test_is_valid_version("0.11.0", "^0.10.0", false);
    test_is_valid_version("0.11.0", "*", true);

    // Anything < 1.0.0 is compatible

    test_is_valid_version("1.0.0", "x.x.x", true);
    test_is_valid_version("1.0.0", "0.x.x", true);
    test_is_valid_version("1.0.0", "0.10.0", false);
    test_is_valid_version("1.0.0", "0.10.2", false);
    test_is_valid_version("1.0.0", "^0.10.2", true);
    test_is_valid_version("1.0.0", "0.10.x", true);
    test_is_valid_version("1.0.0", "^0.10.0", true);
    test_is_valid_version("1.0.0", "1.0.0", true);
    test_is_valid_version("1.0.0", "^1.0.0", true);
    test_is_valid_version("1.0.0", "^2.0.0", false);
    test_is_valid_version("1.0.0", "*", true);
    test_is_valid_version("1.0.0", ">=0.0.1", true);
    test_is_valid_version("1.0.0", ">=0.0.10", true);
    test_is_valid_version("1.0.0", ">=0.10.0", true);
    test_is_valid_version("1.0.0", ">=0.10.1", true);
    test_is_valid_version("1.0.0", ">=1.0.0", true);
    test_is_valid_version("1.0.0", ">=1.1.0", false);
    test_is_valid_version("1.0.0", ">=1.0.1", false);
    test_is_valid_version("1.0.0", ">=2.0.0", false);

    test_is_valid_version("1.0.100", "x.x.x", true);
    test_is_valid_version("1.0.100", "0.x.x", true);
    test_is_valid_version("1.0.100", "0.10.0", false);
    test_is_valid_version("1.0.100", "0.10.2", false);
    test_is_valid_version("1.0.100", "^0.10.2", true);
    test_is_valid_version("1.0.100", "0.10.x", true);
    test_is_valid_version("1.0.100", "^0.10.0", true);
    test_is_valid_version("1.0.100", "1.0.0", false);
    test_is_valid_version("1.0.100", "^1.0.0", true);
    test_is_valid_version("1.0.100", "^1.0.1", true);
    test_is_valid_version("1.0.100", "^2.0.0", false);
    test_is_valid_version("1.0.100", "*", true);

    test_is_valid_version("1.100.0", "x.x.x", true);
    test_is_valid_version("1.100.0", "0.x.x", true);
    test_is_valid_version("1.100.0", "0.10.0", false);
    test_is_valid_version("1.100.0", "0.10.2", false);
    test_is_valid_version("1.100.0", "^0.10.2", true);
    test_is_valid_version("1.100.0", "0.10.x", true);
    test_is_valid_version("1.100.0", "^0.10.0", true);
    test_is_valid_version("1.100.0", "1.0.0", false);
    test_is_valid_version("1.100.0", "^1.0.0", true);
    test_is_valid_version("1.100.0", "^1.1.0", true);
    test_is_valid_version("1.100.0", "^1.100.0", true);
    test_is_valid_version("1.100.0", "^2.0.0", false);
    test_is_valid_version("1.100.0", "*", true);
    test_is_valid_version("1.100.0", ">=1.99.0", true);
    test_is_valid_version("1.100.0", ">=1.100.0", true);
    test_is_valid_version("1.100.0", ">=1.101.0", false);

    test_is_valid_version("2.0.0", "x.x.x", true);
    test_is_valid_version("2.0.0", "0.x.x", false);
    test_is_valid_version("2.0.0", "0.10.0", false);
    test_is_valid_version("2.0.0", "0.10.2", false);
    test_is_valid_version("2.0.0", "^0.10.2", false);
    test_is_valid_version("2.0.0", "0.10.x", false);
    test_is_valid_version("2.0.0", "^0.10.0", false);
    test_is_valid_version("2.0.0", "1.0.0", false);
    test_is_valid_version("2.0.0", "^1.0.0", false);
    test_is_valid_version("2.0.0", "^1.1.0", false);
    test_is_valid_version("2.0.0", "^1.100.0", false);
    test_is_valid_version("2.0.0", "^2.0.0", true);
    test_is_valid_version("2.0.0", "*", true);
}
