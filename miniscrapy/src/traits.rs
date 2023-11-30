/// miniscrapy trait defined according to https://docs.scrapy.org/en/latest/topics/components.html
use std::sync::Weak;

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

/// https://docs.scrapy.org/en/latest/topics/item-pipeline.html
#[async_trait]
pub trait ItemPipeline {
    async fn process_item(&self, item: Value, spider: Weak<dyn Spider>);
    async fn open_spider(&self, spider: Weak<dyn Spider>);
    async fn close_spider(&self, reason: String);
}

/// https://docs.scrapy.org/en/latest/topics/scheduler.html
#[async_trait]
pub trait Scheduler {
    async fn open(&self, spider: Weak<dyn Spider>);
    async fn close(&self, reason: String);
    async fn enqueue_request(&self, request: Request) -> bool;
    async fn has_pending_requests(&self) -> bool;
    async fn next_request(&self) -> Option<Request>;
}

/// https://docs.scrapy.org/en/latest/topics/exporters.html#baseitemexporter
#[async_trait]
pub trait ItemExporter {
    async fn export_item(&self, item: Value);
    async fn start_exporting(&self);
    async fn finish_exporting(&self);
}

/// https://docs.scrapy.org/en/latest/topics/spider-middleware.html#spider-middleware
/// FIXME: reading the docs.
#[async_trait]
pub trait SpiderMiddleware {
    async fn process_spider_input(&self, response: Response, spider: Weak<dyn Spider>);
    async fn process_spider_output(
        &self,
        response: Response,
        spider: Weak<dyn Spider>,
    ) -> Result<(), ()>;
    async fn process_spider_exception(&self);
    async fn process_start_requests(&self, start_requests: Vec<Request>);
}
impl Spider {
    fn start(&self, url: String) {
        let initial_request = Request { url };
        let mut engine = Engine::new(
            self.clone(),
            Scheduler::new(),
            DownloaderMiddleware::new(),
            PipelineMiddleware::new(),
        );
        engine.scheduler.enqueue(initial_request);
        engine.run();
    }
}
fn main() {
    let spider = Spider::new();
    spider.start(String::from("http://example.com"));
