#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub accept: Option<String>,
    pub client: reqwest::Client,
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration::default()
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: "https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery"
                .to_string(),
            user_agent: Some("curl".to_string()),
            client: reqwest::Client::new(),
            accept: Some("Application/json; charset=utf-8; api-version=7.2-preview.1".to_string()),
        }
    }
}
