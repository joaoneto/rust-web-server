use crate::config::Config;

#[derive(Debug)]
pub struct App {
    config: Config,
}

impl App {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    pub fn with_config(&mut self, config: Config) -> &mut Self {
        self.config = config;
        self
    }

    pub fn run(&self) {
        println!("{:?}", self);
    }
}