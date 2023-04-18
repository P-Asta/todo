use std::fs;

use redis::Commands;

pub fn get_web(name: &str) -> String{
    let (html, css, js, all_css, all_js):
        (String, String, String, String, String) = (
        
        fs::read_to_string(format!("web/html/{name}.html")).unwrap_or(String::new()),
        fs::read_to_string(format!("web/css/{name}.css")).unwrap_or(String::new()),
        fs::read_to_string(format!("web/js/{name}.js")).unwrap_or(String::new()),
        fs::read_to_string(format!("web/all/all.css")).unwrap_or(String::new()),
        fs::read_to_string(format!("web/all/all.js")).unwrap_or(String::new())
    );

    format!("{html}<style>{all_css}\n\n{css}</style><script>{js}\n\n{all_js}</script>")
}


pub struct Db{
    cli: redis::Client
}
// github: https://github.com/redis-rs/redis-rs
impl Db {
    pub async fn new() -> Self{
        let path = "redis://:asdf@svc.sel3.cloudtype.app:31672";


        let cli = redis::Client::open(path).unwrap();
        
        
        let mut connection = cli.get_connection().unwrap();
        // connection.set::<&str, &str, ()>("test", "test_data").unwrap();
        let rv: String = connection.get("test").unwrap();
        println!("{rv:?}");
        Self{ cli }
    }
}

