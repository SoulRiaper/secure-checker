#![windows_subsystem = "windows"]

mod view;
mod veda;
mod util;

use std::env;
use view::main_view::render_main_view;
use veda::veda_client::VedaClient;
use web_view::*;
use clap::{arg, command, ArgAction};
use util::local_storage::*;
use util::user_util::*;
use system_shutdown::logout;
use url::Url;

fn main() {

    let matches = command!()
        .arg(arg!(-u --base_url <VALUE>).action(ArgAction::Set))
        .get_matches();
    
    let default_url = "http://username:password@example.com".to_string();
    let base_url = matches.get_one::<String>("base_url").unwrap_or(&default_url);
    let url = Url::parse(base_url).expect("Unable to parse url!");
    let login = url.username();
    let pass  = url.password().unwrap_or("");
    let clean_url = format!("{}://{}{}",
        url.scheme(),
        url.host_str().unwrap_or(""),
        if let Some(port) = url.port() { format!(":{}", port) } else { String::new() },
    );
    println!("url: {}. login: {}.", clean_url, login);

    let mut client: VedaClient = VedaClient::new(clean_url);
    
    let mut is_veda_available = false;

    match client.authenticate(login, pass) {
        Ok(_) => {
            println!("authentication ok");
            is_veda_available = true;
        }
        Err(err) => {
            println!("Failed to authenticate, Try to get acceptance locally or get new, err= {:?}", err);
        }
    };

    let username = match env::var("USERNAME").or_else(|_| env::var("USER")) {
        Ok(res) => res,
        Err(_) => "undefined".to_string()
    };

    match client.get_user_policy_acceptance(username.clone()) {
        Ok(json) => {
            if client.is_acceptance_valid(json) {
                println!("Acceptance OK");
                return
            } else {
                if is_acceptance_info_stored(get_username_hash(username.clone())) {
                    println!("Found local accetance, try to load");
                    if let Ok(json) = get_user_stored_info(get_username_hash(username.clone())) {
                        if client.is_acceptance_valid(json.clone()) {
                        //TODO : Add local acceptance to veda
                        println!("Put local acceptance to veda");
                        }
                    }
                }
                println!("Acceptance end time, Try to get new")
            }
        }
        Err(_) => {
            if let Ok(res) = get_user_stored_info(get_username_hash(username.clone())) {
                if client.is_acceptance_valid(res.clone()) {
                    if is_veda_available {
                        client.put_acceptance_obj(res);
                        remove_acceptance_local_info(username.clone());
                        println!("Put local acceptance to veda");
                    }
                    return
                }
            }
        }
    };

    let formatted_html = render_main_view(username.clone(), "No policy today.".to_string());
    
    web_view::builder()
    .title("Ознакомление с политиками безопасности")
    .content(Content::Html(formatted_html))
    .size(800, 600)
    .resizable(false)
    .user_data(())
    .invoke_handler(move |_webview, _arg| {
        println!("Arg: {}", _arg);
        match _arg {
            "user_accept_policy" => {
                if is_veda_available {
                    if client.put_policy_acceptance_data(username.clone(), get_date_when_acceptance_expiers()).is_ok() {
                       _webview.exit();
                    }
                } else {
                    let acceptance_obj = client.get_acceptance_obj(username.clone(), get_date_when_acceptance_expiers());
                    if write_user_info_localy(acceptance_obj, get_username_hash(username.clone())).is_ok() {
                        _webview.exit();
                    }
                }
            }
            "user_reject_policy" => {
                println!("User reject policy"); 
                _webview.exit();
                let _ = logout();
            }
            _ => {
                println!("Bad input");
            }
        }
        Ok(())
    })
    .frameless(true)
    .build()
    .unwrap()
    .run()
    .unwrap();

}
