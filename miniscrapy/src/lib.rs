use std::sync::Arc;

use async_trait::async_trait;
use reqwest::{Request, Response};
use serde_json::Value;

#[async_trait]
pub trait Spider {
    async fn next(&self) -> Option<Request>;
    async fn parse(&self, response: Response) -> Option<Value>;
}

pub enum DownloaderMiddlewareReturn {
    None,
    Ignore,
    Request(Request),
    Response(Response),
}

#[async_trait]
pub trait DownloaderMiddleware {
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
pub trait ItemMiddleware {
    async fn process_item(&self, item: Value, spider: &dyn Spider);
    async fn open_spider(&self, _spider: &dyn Spider) {}
    async fn close_spider(&self, _spider: &dyn Spider) {}
}

#[async_trait]
pub trait Pipeline {
    async fn open_spider(&self, spider: Arc<dyn Spider>);
    async fn process_item(&self, item: Value);
    async fn close_spider(&self);
}
