pub mod veda_client {

    use openapi::apis::{configuration, default_api};
    use openapi::models::*;
    // use ring::digest::{digest, SHA256};
    // use hex::encode;
    use chrono::{DateTime, Months};
    use chrono::offset::{Utc};

    pub struct VedaClient {
        auth_ticket : String,
        conf: configuration::Configuration,
        user: String,
        uri_addition: String
    }

    impl VedaClient {
        
        pub fn new(base_path: String) -> Self {
            let mut conf = configuration::Configuration::default();
            conf.base_path = base_path;
            return VedaClient{
                auth_ticket: String::default(),
                conf,
                user: String::default(),
                uri_addition: "_paccept".to_string()
            };
        }

        pub fn authenticate(&mut self, login: &str, pass: &str) -> Result<(), String> {
            let res = default_api::authenticate_get(&self.conf, login, pass, None);

            match res {
                Ok(res) => {
                    self.auth_ticket = res.id.unwrap_or_default().to_string();
                    self.user = res.user_uri.unwrap_or_default().to_string();
                    Ok(())
                }
                Err(err) => {
                    Err(err.to_string())
                }
            }
        }

        pub fn get_individual_by_uri(&self, uri: &str) -> Result<serde_json::Value, String> {
            match default_api::get_individual(&self.conf, &self.auth_ticket, uri, None) {
                Ok(json) => {
                    Ok(json)
                }
                Err(err) => {
                    Err(err.to_string())
                }
            }
        }

        pub fn put_policy_acceptance_data(&self, username: String, date: String) -> Result<(), String> {
            let acceptance_uri = format!("d:{}{}", username, self.uri_addition);
            let req_json = serde_json::json!({
                "@": acceptance_uri,
                "rdf:type":[{"data":"cs:SecurityPolicy", "type" : "Uri"}],
                "cs:familiarizedUser": [
                    {
                      "data": username,
                      "type": "String"
                    }
                  ],
                "v-s:date": [
                    {
                        "data": date,
                        "type": "Datetime"
                    }
                ]
            });

            let req: PutIndividualRequest = PutIndividualRequest::new(self.auth_ticket.to_string().clone(), req_json);
            println!("PUT OBJ:{}", req.individual.to_string());
            match default_api::put_individual(&self.conf, &self.auth_ticket, req) {
                Ok(_) => {
                    println!("acceptance seccessfuly put");
                    Ok(())
                },
                Err(err) => {
                    println!("error put");
                    Err(err.to_string())
                }
            }
        }

        pub fn get_user_policy_acceptance(&self, username: String) -> Result<serde_json::Value, String> {
            let acceptance_uri = format!("d:{}{}", username, self.uri_addition);
            match self.get_individual_by_uri(&acceptance_uri) {
                Ok(json) => Ok(json),
                Err(err) => Err(err)
            }
        }

        // //TODO: если понадобится хешировать имя юзера, взять слайс длинной 45-50 
        // fn get_username_hash(&self, username: String) -> String {
        //     return encode(digest(&SHA256, username.as_bytes()).as_ref());
        // }

        pub fn is_acceptance_valid(&self, json: serde_json::Value) -> bool {
            match self.get_individ_property(json, "v-s:date".to_string()) {
                Some(date) => {
                    println!("{}",date.as_str().trim());
                    let acceptance_date = DateTime::parse_from_str(date.as_str().trim(), "\"%Y-%m-%dT%H:%M:%SZ\"");
                    match acceptance_date {
                        Ok(res) => {
                            println!("trying to compare strings");
                            let date = res.with_timezone(&Utc);
                            let now = Utc::now();
                            return now > date;
                        }
                        Err(_) => {
                            println!("Cant parse string");
                            false
                        }
                    }
                }
                None => false
            }
        }

        pub fn get_individ_property(&self, json: serde_json::Value, property_name: String) -> Option<String> {
            match json.get(property_name) {
                Some(date_bundle) => {
                    match date_bundle.get(0) {
                        Some(date_obj) => {
                            match date_obj.get("data") {
                                Some(date) => Some(date.to_string()),
                                None => None
                            }
                        }
                        None => None
                    }
                }
                None => None
            }
        }

        pub fn get_now_date_when_acceptance_expiers_string(&self) -> String {
            let now = Utc::now();
            let mounths = Months::new(3);
            match now.checked_add_months(mounths) {
                Some(date) => {
                    date.format("%Y-%m-%dT%H:%M:%SZ").to_string()
                }
                None => String::default()
            }
        }

    }
}