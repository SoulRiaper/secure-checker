#![windows_subsystem = "windows"]

mod view;
mod veda;

use web_view::*;
use std::env;
use view::view::render_main_view;
use veda::veda_client::VedaClient;

#[tokio::main] 
async fn main() {
    let mut client = VedaClient::new("http://localhost:8080".to_string());
    client.authenticate("karpovrt", "a665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3").await;

    let username = match env::var("USERNAME").or_else(|_| env::var("USER")) {
        Ok(res) => res,
        Err(_) => "undefined".to_string()
    };

    let formatted_html = render_main_view(username, format!("No policy today. auth token is: {}, for user: {}", client.get_auth_ticket(), client.get_user_account()));
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
