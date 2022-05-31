mod app;
mod config;

fn home_page() {
    println!("Welcome to the home page!");
}

#[tokio::main]
async fn main() {    
    let mut app = app::App::new();

    app.with_config(config::Config {
        host: "0.0.0.0".to_string(),
        ..Default::default()
    });

    tokio::spawn(
        async move {
            app
                .with_route(app::Route::new(
                    "/".to_string(),
                    "GET".to_string(),
                    home_page,
                ))
                .run();
        }
    )
    .await
    .unwrap();
}
