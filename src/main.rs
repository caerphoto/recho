use axum::{
    Router,
    extract::OriginalUri,
    http::header::HeaderMap,
};
use colored::Colorize;

async fn echo(uri: OriginalUri, headers: HeaderMap, body: String) {
    if body.is_empty() {
        log::info!("Request path: {}\nHeaders:\n{}\n(no request body)",
            format!("{}", uri.path()).magenta(),
            format!("{:#?}", headers).cyan())
    } else {
        log::info!("Request path: {}\nHeaders:\n{}\nBody:\n{}",
            format!("{}", uri.path()).magenta(),
            format!("{:#?}", headers).cyan(),
            format!("{}", body).green());
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = Router::new()
        // .route("/", any(echo))
        .fallback(echo);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();

    log::info!("Starting up...");
    axum::serve(listener, app).await.unwrap();
}
