use vscode_derive::api;

#[api(Default)]
pub struct ExtensionProperty {
    pub key: String,
    pub value: String,
}
