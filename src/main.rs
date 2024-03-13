#![windows_subsystem = "windows"]

mod view;

use web_view::*;
use std::env;
use view::view::render_main_view;

fn main() {
    let username = match env::var("USERNAME").or_else(|_| env::var("USER")) {
        Ok(res) => res,
        Err(_) => "undefined".to_string()
    };
    let formatted_html = render_main_view(username, "No policy today".to_string());
    println!("{}",formatted_html);

    web_view::builder()
        .title("Привет!")
        .content(Content::Html(formatted_html))
        .size(800, 600)
        .resizable(false)
        .user_data(())
        .invoke_handler(|_webview, _arg| {
            println!("Arg: {}", _arg);
            match _arg {
                "user_accept_policy" => {
                    println!("User accept policy");
                }
                "user_reject_policy" => {
                    println!("User reject policy"); 
                    _webview.exit();
                }
                _ => {
                    println!("Bad input");
                }
            }
            Ok(())
        })
        .build()
        .unwrap()
        .run()
        .unwrap();

        
}
