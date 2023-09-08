use std::collections::HashMap;

pub fn main() -> String {
    String::from("index.js")
}

/// see https://docs.npmjs.com/cli/v8/configuring-npm/package-json#default-values
pub fn scripts() -> HashMap<String, String> {
    HashMap::from([
        ("start".to_owned(), "node server.js".to_owned()),
        ("install".to_owned(), "node-gyp rebuild".to_owned()),
    ])
}

pub fn r#type() -> String {
    String::from("commonjs")
}
