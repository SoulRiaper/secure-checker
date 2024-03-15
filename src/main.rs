#![windows_subsystem = "windows"]

mod view;
mod veda;

use std::env;
use view::view::render_main_view;
use veda::veda_client::VedaClient;
use web_view::*;

fn main() {
    let mut client: VedaClient = VedaClient::new("http://localhost:8080".to_string());
    let login = "karpovrt";
    let pass  = "a665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3";
    match client.authenticate(login, pass) {
        Ok(_) => {
            println!("authentication ok");
        } 
        Err(_) => {
            println!("Failed to authenticate, Exit");
            return;
        }
    };

    let username = match env::var("USERNAME").or_else(|_| env::var("USER")) {
        Ok(res) => res,
        //need to logout user
        Err(_) => "undefined".to_string()
    };

    match client.get_user_policy_acceptance(username.clone()) {
        Ok(json) => {
            if client.is_acceptance_valid(json) {
                println!("Acceptance OK");
                return;
            } else {
                println!("Acceptance end time, try to get new")
            }
        }
        Err(_) => println!("Acceptance not found. Tru to show main")
    };

    let formatted_html = render_main_view(username.clone(), format!("No policy today."));
    
    web_view::builder()
    .title("Привет!")
    .content(Content::Html(formatted_html))
    .size(800, 600)
    .resizable(false)
    .user_data(())
    .invoke_handler(move |_webview, _arg| {

        println!("Arg: {}", _arg);
        match _arg {
            "user_accept_policy" => {
                let user_data = username.clone();
                println!("User {} accept policy", user_data);
                match client.put_policy_acceptance_data(username.clone(), client.get_now_date_when_acceptance_expiers_string()) {
                    Ok(_) => {
                        _webview.exit();
                    }
                    Err(_) => ()
                }
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
