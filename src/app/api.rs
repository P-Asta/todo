use std::fs;

use axum::{extract::Path, response::Html};

pub async fn asset(Path(name): Path<String>) -> Html<String>{
    Html(fs::read_to_string(format!("assets/{name}")).unwrap_or("404 Not Found".to_string()))
}