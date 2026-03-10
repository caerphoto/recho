use std::collections::HashMap;
use axum::{
    Router,
    extract::{Query, OriginalUri},
    http::{Method, header::HeaderMap},
};
use colored::Colorize;

async fn echo(uri: OriginalUri, method: Method, Query(params): Query<HashMap<String, String>>, headers: HeaderMap, body: String) {
    if body.is_empty() {
        log::info!("Request to: {}\nQuery:\n{}\nHeaders:\n{}\n(no request body)",
            format!("{} {}", method, uri.path()).magenta(),
            format!("{:#?}", params).yellow(),
            format!("{:#?}", headers).cyan())
    } else {
        log::info!("Request path: {}\nQuery:\n{}\nHeaders:\n{}\nBody:\n{}",
            format!("{} {}", method, uri.path()).magenta(),
            format!("{:#?}", params).yellow(),
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
