use std::fmt::{
    Debug,
    Formatter,
    Result,
    Display,
};

#[derive(Debug)]
pub struct Request {
    url: String,
}
impl Request {
    pub fn new() -> Self {
        Self {
            url: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct Response {
    url: String,
}
impl Response {
    pub fn new() -> Self {
        Self {
            url: String::new(),
        }
    }
}

impl Display for App {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{ name: {} }}", self.name)
    }
}
pub struct App {
    pub name: String,
    middlewares: Vec<Box<dyn Fn(&mut Request, &mut Response)>>,
    pub req: Request,
    pub res: Response,
}

impl App {
    pub fn new(name: String) -> Self {
        Self {
            name,
            middlewares: Vec::new(),
            req: Request::new(),
            res: Response::new(),
        }
    }

    pub fn parse_request(&mut self, raw_request: String) {
        println!("> App::parse_request {}", raw_request);
        self.req.url = raw_request;
    }

    pub fn middleware<F: Fn(&mut Request, &mut Response) + 'static>(&mut self, callback: F) {
        self.middlewares.push(Box::new(callback));
    }

    // todo, make this async with future::join_all
    pub fn call(&mut self) {
        for callback in self.middlewares.iter() {
            callback(&mut self.req, &mut self.res);
        }
    }
}