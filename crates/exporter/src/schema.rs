// @generated automatically by Diesel CLI.

diesel::table! {
    marketplace (name, publisher, version, engine, platform, assert_url) {
        name -> Text,
        publisher -> Text,
        version -> Text,
        engine -> Text,
        platform -> Text,
        assert_url -> Text,
        hash -> Nullable<Text>,
        is_prerelease -> Bool,
    }
}
