use axum::response::Html;
use std::fs;
pub fn get_web(name: &str) -> String{
    let (html, css, js, all_css, all_js):
        (String, String, String, String, String) = (
        
        fs::read_to_string(format!("web/html/{name}.html")).unwrap_or(String::new()),
        fs::read_to_string(format!("web/css/{name}.css")).unwrap_or(String::new()),
        fs::read_to_string(format!("web/js/{name}.js")).unwrap_or(String::new()),
        fs::read_to_string(format!("web/all/all.css")).unwrap_or(String::new()),
        fs::read_to_string(format!("web/all/all.js")).unwrap_or(String::new())
    );

    format!("{html}<style>{css}\n\n{all_css}</style><script>{js}\n\n{all_js}</script>")
}