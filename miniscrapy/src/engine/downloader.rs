use reqwest::{Request, Response};

use crate::{DownloaderMiddleware, DownloaderMiddlewareReturn, Spider};

pub(crate) struct Downloader {
    downloader_middleware: Vec<Box<dyn DownloaderMiddleware>>,
    spider: Box<dyn Spider>,
}

impl Downloader {
    pub(crate) async fn handle_request(&self, req: Request) -> DownloaderMiddlewareReturn {
        let req = Some(req);
        for middleware in &self.downloader_middleware {
            let Some(req) = req else {
                continue;
            };

            match middleware.process_request(req, &*self.spider).await {
                DownloaderMiddlewareReturn::None => todo!(),
                DownloaderMiddlewareReturn::Request(_) => todo!(),
                DownloaderMiddlewareReturn::Response(_) => todo!(),
            };
        }
        // using watchdog
        if let Some(_req) = req {
            todo!()
        }

        todo!()
    }

    pub(crate) async fn handle_response(&self, _rep: Response) -> DownloaderMiddlewareReturn {
        todo!()
    }

    pub(crate) async fn run(&mut self) {
        // run handle_request task
        // run handle_response task
        todo!()
    }
}
