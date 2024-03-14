pub mod veda_client {

    use openapi::apis::{configuration, default_api};

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

        pub async fn authenticate(&mut self, login: &str, pass: &str) {
            let res = default_api::authenticate_get(&self.conf, login, pass, None).await;

            match res {
                Ok(res) => {
                    self.auth_ticket = res.id.unwrap_or_default().to_string();
                    self.user = res.user_uri.unwrap_or_default().to_string();
                }
                Err(err) => {
                    println!("Login error: {}.Exit.", err.to_string());
                }
            }
        }


        pub fn get_auth_ticket(&self) -> &String {
            &self.auth_ticket
        }

        pub fn get_user_account(&self) -> &String {
            &self.user
        }

        pub async fn get_individual_by_uri(&self, uri: &str) {
        }

    }
}