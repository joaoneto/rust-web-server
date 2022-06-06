mod app;
mod config;

fn home_page(_req: &app::Request) -> Result<app::Response, app::HttpError> {
    Ok(app::Response {
        status: 200,
        headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
        body: Some(String::from("Welcome to the home page!")),
    })
}

fn not_found(_req: &app::Request) -> Result<app::Response, app::HttpError> {
    Err(app::HttpError::NotFound)
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
                .with_route(app::Route::new(
                    "/err".to_string(),
                    "GET".to_string(),
                    not_found,
                ))
                .run();
        }
    )
    .await
    .unwrap();
}
