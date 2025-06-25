// @generated automatically by Diesel CLI.

diesel::table! {
    marketplace (name, publisher, version, engine, platform) {
        name -> Text,
        publisher -> Text,
        version -> Text,
        engine -> Text,
        platform -> Text,
        hash -> Nullable<Text>,
        is_prerelease -> Bool,
        url -> Nullable<Text>,
    }
}
