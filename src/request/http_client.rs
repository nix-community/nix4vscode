use log::error;

use crate::{
    config::Extension,
    data_struct::{self, IRawGalleryExtensionsResult, TargetPlatform},
    error::Error,
};

use super::Query;

#[derive(Debug, Clone)]
pub struct HttpClient {
    pub client: reqwest::Client,
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

    async fn inner_get_extension_target_platform(
        &self,
        publisher_name: String,
        extension_name: String,
    ) -> anyhow::Result<IRawGalleryExtensionsResult> {
        let query = Query::create_search(publisher_name, extension_name);
        let body = serde_json::to_string(&query)?;
        let txt = self
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
            .await?;
        txt.results.into_iter().next().ok_or(Error::Unknown.into())
    }

    pub async fn get_extension_target_platform(
        &self,
        publisher_name: String,
        extension_name: String,
    ) -> Vec<TargetPlatform> {
        match self
            .inner_get_extension_target_platform(publisher_name, extension_name)
            .await
        {
            Ok(res) => res.get_target_platform(),
            Err(err) => {
                error!("Error happend when get target_platform: {err}");
                vec![]
            }
        }
    }
}
