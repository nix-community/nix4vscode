use crate::{config::Extension, data_struct};

use super::Query;

#[derive(Debug, Clone)]
pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> anyhow::Result<Self> {
        let client = reqwest::Client::builder().gzip(true).build()?;
        Ok(Self { client })
    }

    pub async fn get_extension_response(
        &self,
        extensions: &[Extension],
    ) -> anyhow::Result<data_struct::IRawGalleryQueryResult> {
        let query = Query::new(extensions);
        let body = serde_json::to_string(&query)?;
        Ok(self
            .client
            .post("https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery")
            .header(
                "Accept",
                "Application/json; charset=utf-8; api-version=7.2-preview.1",
            )
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?
            .json::<data_struct::IRawGalleryQueryResult>()
            .await?)
    }
}
