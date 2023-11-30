use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
};

struct Request {
    url: String,
    // 在这里添加 Request 的其他属性，例如优先级、回调函数等
}

struct Scheduler {
    queue: Arc<Mutex<VecDeque<Request>>>,
    filter: Arc<Mutex<HashMap<String, bool>>>,
}

impl Scheduler {
    fn new() -> Scheduler {
        Scheduler {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            filter: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn add_request(&self, request: Request) {
        let mut filter = self.filter.lock().unwrap();
        if filter.contains_key(&request.url) {
            println!("Duplicate request: {}", request.url);
            return;
        }
        filter.insert(request.url.clone(), true);
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(request);
    }

    fn get_request(&self) -> Option<Request> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front()
    }
}

fn main() {
    let s = Scheduler::new();
    s.add_request(Request {
        url: String::from("http://example.com"),
    });
    s.add_request(Request {
        url: String::from("http://example.com"),
    }); // 这个请求会被过滤掉
    if let Some(request) = s.get_request() {
        println!("Processing request: {}", request.url);
    }
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
