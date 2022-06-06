use std::sync::Arc;
use crate::config::Config;

#[derive(Clone)]
pub struct Request {
    pub config: Arc<Config>,
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

#[derive(Debug)]
pub enum HttpError {
    NotFound,
    BadRequest,
    InternalServerError,
}

#[derive(Clone)]
pub struct Route {
    path: String,
    method: String,
    handler: Arc<dyn Fn(&Request,) -> Result<Response, HttpError> + Send + Sync + 'static>,
}

impl Route {
    pub fn new(path: String, method: String, handler: impl Fn(&Request,) -> Result<Response, HttpError> + Send + Sync + 'static) -> Self {
        Self {
            path,
            method,
            handler: Arc::new(handler),
        }
    }
}

#[derive(Clone)]
pub struct App {
    config: Config,
    routes: Vec<Route>,
}

impl App {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            routes: Vec::new(),
        }
    }

    pub fn with_config(&mut self, config: Config) -> &mut Self {
        self.config = config;
        self
    }

    pub fn with_route(&mut self, route: Route) -> &mut Self {
        self.routes.push(route);
        self
    }

    pub fn run(&mut self) {
        let req = Request {
            config: Arc::new(self.config.clone()),
            url: String::from(""),
            method: String::from(""),
            headers: Vec::new(),
            body: None,
        };

        self
            .routes
            .iter_mut()
            .for_each(|route| {
                println!("{} {}", route.method, route.path);
                let response = (route.handler)(&req);
                if response.is_ok() {
                    println!("{:?}", response.unwrap());
                } else {
                    let res = Response {
                        status: 404,
                        headers: Vec::new(),
                        body: Some(std::fmt::format(format_args!("{:?}", response.unwrap_err()))),
                    };
                    println!("{:?}", res);
                }
            });
    }
}