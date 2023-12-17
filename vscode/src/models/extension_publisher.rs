use vscode_derive::api;

#[api(Default)]
pub struct ExtensionPublisher {
    pub display_name: String,
    pub publisher_id: String,
    pub publisher_name: String,
    pub domain: Option<String>,
    pub is_domain_verified: bool,
}
