use axum::{response::Html, extract::Path};

use super::builtin::*;

pub async fn index() -> Html<String>{
    Html(get_web("index"))
}

pub async fn todo_app(Path(name): Path<String>) -> Html<String>{
    let app = get_web("todo");

    Html(
        app
            .replace("{{name}}", &name)
    )
}

pub async fn sign_up() -> Html<String>{
    let app = get_web("sign_up");

    Html(
        app
    )
}
