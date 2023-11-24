use async_trait::async_trait;
use reqwest::{Request, Response};
use serde_json::Value;

use crate::MiniScrapyResult;

#[async_trait]
pub trait Spider {
    fn name(&self) -> &str;
    async fn next_request(&self) -> MiniScrapyResult<Option<Request>>;
    async fn parse(&self, response: Response) -> Option<Value>;
}

pub enum DownloaderMiddlewareReturn {
    None,
    Request(Request),
    Response(Response),
}

#[async_trait]
pub trait DownloaderMiddleware: Sync {
    async fn process_request(
        &self,
        request: Request,
        _spider: &dyn Spider,
    ) -> DownloaderMiddlewareReturn {
        DownloaderMiddlewareReturn::Request(request)
    }
    async fn process_response(
        &self,
        response: Response,
        _spider: &dyn Spider,
    ) -> DownloaderMiddlewareReturn {
        DownloaderMiddlewareReturn::Response(response)
    }
}

#[async_trait]
pub trait ItemPipeline {
    async fn process_item(&self, item: Value, spider: &dyn Spider);
}
