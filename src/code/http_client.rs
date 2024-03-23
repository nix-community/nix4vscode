use crate::{
    code::{
        self, IQueryState, IRawGalleryExtensionsResult, IRawGalleryQueryResult, TargetPlatform,
    },
    config::Extension,
    error::Error,
};

use super::Query;
use anyhow::anyhow;
use tracing::*;

#[derive(Debug, Clone)]
pub struct HttpClient {
    pub client: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> anyhow::Result<Self> {
        let client = reqwest::Client::builder().gzip(true).build()?;
        Ok(Self { client })
    }

    pub async fn query_one(
        &self,
        publisher: &str,
        name: &str,
    ) -> anyhow::Result<code::IRawGalleryExtension> {
        let mut res = IRawGalleryQueryResult::default();
        let mut page_number: u64 = 1;
        loop {
            let query = Query::create_one(publisher, name, page_number);
            let body = serde_json::to_string(&query)?;
            trace!("send request: {body}");
            let mut response = self
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
                .json::<IRawGalleryQueryResult>()
                .await?;

            if response.results[0].extensions.is_empty() {
                break;
            }

            res.results.append(&mut response.results);
            page_number += 1;
        }

        res.results.iter_mut().for_each(|item| {
            item.extensions.iter_mut().for_each(|item| {
                item.extension_name = item.extension_name.to_lowercase();
                item.publisher.publisher_name = item.publisher.publisher_name.to_lowercase();
            })
        });

        let mut res: Vec<_> = res
            .results
            .into_iter()
            .flat_map(|item| item.extensions)
            .filter(|item| {
                item.extension_name == name && item.publisher.publisher_name == publisher
            })
            .collect();

        if res.is_empty() {
            return Err(anyhow!(format!("cannot get {publisher}.{name}")));
        }

        let mut v = res.pop().unwrap();

        res.into_iter()
            .for_each(|item| v.versions.extend(item.versions));

        Ok(v)
    }

    pub async fn get_extension_response(
        &self,
        extensions: &[Extension],
    ) -> anyhow::Result<code::IRawGalleryQueryResult> {
        let mut results = IRawGalleryQueryResult::default();
        let extension_count: u64 = extensions.len() as u64;
        let mut page_number: u64 = 1;
        loop {
            let query = Query::new(extensions, page_number);
            let body = serde_json::to_string(&query)?;
            trace!("send request: {body}");
            let mut response = self
                .client
                .post("https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery")
                .header(
                    "Accept",
                    "Application/json; charset=utf-8; api-version=7.2-preview.1",
                )
                .header("Content-Type", "application/json")
                .body(body.clone())
                .send()
                .await?
                .json::<IRawGalleryQueryResult>()
                .await?;

            if response.results.is_empty() {
                break;
            }

            results.results.append(&mut response.results);
            if page_number * IQueryState::DEFAULT_PAGE_SIZE >= extension_count {
                break;
            }

            page_number += 1;
        }
        Ok(results)
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
            .json::<code::IRawGalleryQueryResult>()
            .await?;
        txt.results.into_iter().next().ok_or(Error::Unknown.into())
    }

    pub async fn get_extension_target_platform(
        &self,
        publisher_name: String,
        extension_name: String,
    ) -> Vec<TargetPlatform> {
        trace!("get target_platform of {publisher_name}.{extension_name}");
        match self
            .inner_get_extension_target_platform(publisher_name, extension_name)
            .await
        {
            Ok(res) => {
                let i: Vec<_> = res
                    .get_target_platform()
                    .into_iter()
                    .filter(|item| !matches!(item, TargetPlatform::Unknown))
                    .collect();

                let j: Vec<_> = i
                    .iter()
                    .filter(|item| {
                        !matches!(*item, TargetPlatform::Web | TargetPlatform::Universal)
                    })
                    .copied()
                    .collect();

                if !j.is_empty() {
                    j
                } else {
                    i
                }
            }
            Err(err) => {
                error!("Error happend when get target_platform: {err}");
                vec![]
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_require_one_extension() {
        let client = HttpClient::new().unwrap();

        for i in [
            ("ms-toolsai", "jupyter"),
            ("ms-ceintl", "vscode-language-pack-zh-hans"),
        ] {
            let v = client.query_one(i.0, i.1).await.unwrap();
            assert_eq!(v.extension_name, i.1.to_lowercase());
        }
    }
}
