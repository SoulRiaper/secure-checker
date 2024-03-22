pub mod veda_client {

    use openapi::apis::{configuration, default_api};
    use openapi::models::*;
    use chrono::{DateTime, Months};
    use chrono::offset::Utc;
    use serde_json::Value;

    pub struct VedaClient {
        auth_ticket : String,
        conf: configuration::Configuration,
        user: String,
        uri_addition: String
    }

    impl VedaClient {
        
        pub fn new(base_path: String) -> Self {
            let conf = configuration::Configuration{
                base_path,
                ..Default::default()
            };
            VedaClient{
                auth_ticket: String::default(),
                conf,
                user: String::default(),
                uri_addition: "_paccept".to_string()
            }
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
            let req_json = self.get_acceptance_obj(username, date);

            let req: PutIndividualRequest = PutIndividualRequest::new(self.auth_ticket.to_string().clone(), req_json);
            println!("PUT OBJ:{}", req.individual);
            if default_api::put_individual(&self.conf, &self.auth_ticket, req).is_ok() {Ok(())}
            else { Err("cant put data".to_string()) }
        }

        pub fn put_acceptance_obj(&self, json : Value) {
            let req: PutIndividualRequest = PutIndividualRequest::new(self.auth_ticket.to_string().clone(), json);

            if default_api::put_individual(&self.conf, &self.auth_ticket, req).is_ok() {}
        }

        pub fn get_acceptance_obj(&self, username: String, date: String) -> Value {
            let acceptance_uri = format!("d:{}{}", username, self.uri_addition);
            serde_json::json!({
                "@": acceptance_uri,
                "rdf:type":[{"data":"v-s:SecurityPolicy", "type" : "Uri"}],
                "v-s:familiarizedUser": [
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
            })
        }

        pub fn get_user_policy_acceptance(&self, username: String) -> Result<serde_json::Value, String> {
            let acceptance_uri = format!("d:{}{}", username, self.uri_addition);
            match self.get_individual_by_uri(&acceptance_uri) {
                Ok(json) => Ok(json),
                Err(err) => Err(err)
            }
        }

        pub fn is_acceptance_valid(&self, json: serde_json::Value) -> bool {
            match self.get_individ_property(json, "v-s:date".to_string()) {
                Some(date) => {
                    let date_str = date.as_str().trim();
                    let acceptance_date = DateTime::parse_from_rfc3339(date_str);

                    if let Ok(res) = acceptance_date {
                            let date = res.with_timezone(&Utc);
                            let now = Utc::now();
                            println!("date: {} now: {}", date, now);
                            now < date
                        } else {false}
                }
                None => false
            }
        }

        pub fn get_individ_property(&self, json: serde_json::Value, property_name: String) -> Option<String> {
            match json.get(property_name) {
                Some(date_bundle) => {
                    match date_bundle.get(0) {
                        Some(date_obj) => {
                            date_obj.get("data").map(|date| {
                                format!("{}", date.as_str().unwrap())
                            })
                        }
                        None => None
                    }
                }
                None => None
            }
        }

        pub fn get_date_when_acceptance_expiers(&self) -> String {
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