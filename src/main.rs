#![windows_subsystem = "windows"]

use web_view::*;
use std::env;

fn main() {
    let username = match env::var("USERNAME").or_else(|_| env::var("USER")) {
        Ok(res) => res,
        Err(_) => "undefined".to_string()
    };
    web_view::builder()
        .title("Привет!")
        .content(Content::Html(format!("<h1>Привет, {}</h1>", username)))
        .size(320, 480)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
