mod app;
mod config;

fn main() {
    let mut app = app::App::new();

    app
        .with_config(config::Config {
            host: "0.0.0.0".to_string(),
            ..Default::default()
        })
        .run();
}
