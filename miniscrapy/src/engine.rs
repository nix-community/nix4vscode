mod downloader;
mod item_pipeline;

use downloader::*;
use item_pipeline::*;

use crate::*;

#[derive(Default)]
pub struct EngineBuilder {
    spiders: Vec<Box<dyn Spider>>,
    downloader_middleware: Vec<Box<dyn DownloaderMiddleware>>,
    item_pipeline: Option<Box<dyn ItemPipeline>>,
}

impl EngineBuilder {
    pub fn spider(mut self, spider: Box<dyn Spider>) -> Self {
        self.spiders.push(spider);
        self
    }

    pub fn downloader_middleware(mut self, middleware: Box<dyn DownloaderMiddleware>) -> Self {
        self.downloader_middleware.push(middleware);
        self
    }

    pub fn item_pipeline(mut self, pipeline: Box<dyn ItemPipeline>) -> Self {
        let _ = std::mem::replace(&mut self.item_pipeline, Some(pipeline));
        self
    }

    pub fn build(mut self) -> Engine {
        Engine {
            spiders: self.spiders,
        }
    }
}

pub struct Engine {
    spiders: Vec<Box<dyn Spider>>,
}

impl Engine {
    fn start(&mut self) {
        todo!()
    }
}
