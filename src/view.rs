pub mod main_view {

    use askama::Template;

    #[derive(Template)]
    #[template(path = "index.html")]
    struct MainTemplate {
        user: String,
        policy_text: String,
    }

    pub fn render_main_view(user: String, policy_text: String) -> String {
        let template = MainTemplate {
            user,
            policy_text
        };
        template.render().ok().unwrap_or_default()
    }
}
