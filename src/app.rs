use std::sync::Arc;
use crate::config::Config;

#[derive(Clone)]
pub struct Route {
    path: String,
    method: String,
    handler: Arc<dyn Fn() + Send + Sync + 'static>,
}

impl Route {
    pub fn new(path: String, method: String, handler: impl Fn() + Send + Sync + 'static) -> Self {
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
        self
            .routes
            .iter_mut()
            .for_each(|route| {
                println!("{} {}", route.method, route.path);
                (route.handler)();
            });
    }
}