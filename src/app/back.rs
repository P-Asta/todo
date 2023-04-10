
use axum::{*, response::Html};
use serde::Deserialize;

use super::builtin::get_web;


#[derive(Deserialize, Debug)]
pub struct SignUp{
    pub name: String,
    pub pw: String,
    pub rpw: String
}

pub async fn sign_up(extract::Form(q): extract::Form<SignUp>) -> Html<String>{
    if q.pw != q.rpw{
        return Html(get_web("sign_up").replace("{{param}}", &format!("?n={}&p={}&r={}", q.name, q.pw, q.rpw)).replace("{{error}}", "1"))
    }
    Html(get_web("sign_up").replace("{{param}}", &format!("?n={}&p={}&r={}", q.name, q.pw, q.rpw)).replace("{{error}}", "0"))
}