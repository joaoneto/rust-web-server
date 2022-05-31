#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub host: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            port: 8080,
            host: "127.0.0.1".to_string(),
        }
    }
}
