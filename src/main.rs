mod app;
use axum::{*, routing::{get, post}, http::StatusCode, handler::HandlerWithoutStateExt};
use app::*;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let api = Router::new()
        .route("/assets/*name", get(api::asset))
        .route("/assets/*name", post(api::asset))
        ;

    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Not found")
    }
    
    let serve_dir = ServeDir::new("/assets").not_found_service(handle_404.into_service());
    let app = Router::new()
        .nest("/api", api)
        .route("/", get(front::index))
        .route("/todo/:name", get(front::todo_app))
        .route("/sign_up", get(front::sign_up))
        .route("/sign_up", post(back::sign_up))
        .fallback_service(serve_dir)
    
        ;

    let server = Server::bind(&"0.0.0.0:5023".parse().unwrap())
        .serve(app.into_make_service())
        ;

    server.await.unwrap();
}
