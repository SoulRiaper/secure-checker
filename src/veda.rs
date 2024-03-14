pub mod veda_client {

    use std::fmt::Error;
    use openapi::apis::{configuration, default_api};
    use openapi::models::*;
    use std::hash::{DefaultHasher, Hasher};

    pub struct VedaClient {
        auth_ticket : String,
        conf: configuration::Configuration,
        user: String
    }

    impl VedaClient {
        
        pub fn new(base_path: String) -> Self {
            let mut conf = configuration::Configuration::default();
            conf.base_path = base_path;
            return VedaClient{
                auth_ticket: String::default(),
                conf,
                user: String::default()
            };
        }

        pub fn default() -> Self {
            return VedaClient{
                auth_ticket: String::default(),
                conf: configuration::Configuration::default(),
                user: String::default()
            }
        }

        pub fn authenticate(&mut self, login: &str, pass: &str) -> Result<(), Error> {
            let res = default_api::authenticate_get(&self.conf, login, pass, None);

            match res {
                Ok(res) => {
                    self.auth_ticket = res.id.unwrap_or_default().to_string();
                    self.user = res.user_uri.unwrap_or_default().to_string();
                    Ok(())
                }
                Err(_) => {
                    Err(Error::default())
                }
            }
        }


        pub fn get_auth_ticket(&self) -> &String {
            &self.auth_ticket
        }

        pub fn get_user_account(&self) -> &String {
            &self.user
        }

        pub fn get_individual_by_uri(&self, uri: &str) -> Option<String> {
            match default_api::get_individual(&self.conf, &self.auth_ticket, uri, None) {
                Ok(json) => {
                    Some(json.to_string())
                }
                Err(_) => {
                    None
                }
            }
        }

        pub fn put_policy_data(&self, username: String, date: String) {
            let acceptance_uri = format!("d:{}_pf", self.get_username_hash(username.clone()));
            let req_json = serde_json::json!({
                "@": acceptance_uri,
                "rdf:type":[{"data":"v-s:SecurityPolicy","type":"Uri"}],
                "v-s:familiarizedUser": [
                    {
                      "data": username,
                      "type": "String"
                    }
                  ],
            });

            let req: PutIndividualRequest = PutIndividualRequest::new(self.auth_ticket.to_string().clone(), req_json);
            println!("ticket: {} add new acceptance uri: {}",req.ticket,  acceptance_uri);
            match default_api::put_individual(&self.conf, req) {
                Ok(_) => println!("acceptance seccessfuly put"),
                Err(_) => println!("error put")
            };
        }

        fn get_username_hash(&self, username: String) -> u64 {
            let mut hasher = DefaultHasher::new();
            hasher.write(&username.as_bytes());
            return hasher.finish();
        }

    }
}