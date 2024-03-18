#![windows_subsystem = "windows"]

mod view;
mod veda;
mod local_storage;

use std::env;
use view::view::render_main_view;
use veda::veda_client::VedaClient;
use web_view::*;
use clap::{arg, command, ArgAction};
use local_storage::storage::*;
use std::process::Command;

fn main() {

    let matches = command!()
        .arg(arg!(-u --base_uri <VALUE>).required(true).action(ArgAction::Set))
        .get_matches();
    
    let base_uri = matches.get_one::<String>("base_uri").expect("required");

    let mut client: VedaClient = VedaClient::new(base_uri.clone());
    let login = "karpovrt";
    let pass  = "a665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3";
    let mut is_veda_available = false;

    match client.authenticate(login, pass) {
        Ok(_) => {
            println!("authentication ok");
            is_veda_available = true;
        }
        Err(_) => {
            println!("Failed to authenticate, Try to store acceptance locally");
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
                return;
            } else {
                if is_acceptance_info_stored(username.clone()) {
                    println!("Found local accetance, try to load");
                    match get_user_stored_info(username.clone()) {
                        Ok(json) => {
                            if client.is_acceptance_valid(json.clone()) {
                                println!("Put local acceptance to veda");
                                
                            }
                        }
                        Err(_) => ()
                    }
                }
                println!("Acceptance end time, Try to get new")
            }
        }
        Err(_) => {
            match get_user_stored_info(username.clone()) {
                Ok(res) => {
                    if client.is_acceptance_valid(res.clone()) {
                        println!("Locally stored acceptance valid, veda: {}", is_veda_available);
                        if is_veda_available {
                            client.put_acceptance_obj(res);
                            remove_acceptance_local_info(username.clone());
                            println!("Put local acceptance to veda");
                        }
                        return;
                    }
                }
                Err(_) => {
                    println!("Acceptance not found, Try to get new")
                }
            }
        }
    };

    let formatted_html = render_main_view(username.clone(), format!("No policy today."));
    
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
                    match client.put_policy_acceptance_data(username.clone(), client.get_date_when_acceptance_expiers()) {
                        Ok(_) => {
                            _webview.exit();
                        }
                        Err(_) => ()
                    }
                } else {
                    let acceptance_obj = client.get_acceptance_obj(username.clone(), client.get_date_when_acceptance_expiers());
                    match write_user_info_localy(acceptance_obj, username.clone()) {
                        Ok(_) => {
                            _webview.exit();
                        }
                        Err(_) => ()
                    }
                }
                
            }
            "user_reject_policy" => {
                println!("User reject policy"); 
                _webview.exit();
                log_out_user();
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

pub fn log_out_user() -> std::io::Result<()> {
    let mut command = Command::new("shutdown.exe");
    command.args(&["/l"]);
    let result = command.status()?;
    Ok(())
  } 
