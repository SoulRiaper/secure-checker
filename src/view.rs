pub mod main_view {

    use askama::Template;
    use std::fs;

    #[cfg(target_os = "windows")]
    #[derive(Template)]
    #[template(path = "win.index.html")]
    struct MainTemplate {
        styles: String,
        user: String,
        policy_text: String,
    }

    #[cfg(not(any(target_os = "windows")))]
    #[derive(Template)]
    #[template(path = "index.html")]
    struct MainTemplate {
        styles: String,
        user: String,
        policy_text: String,
    }

    pub fn render_main_view(user: String, policy_text: String) -> String {
        let styles = fs::read_to_string("styles.css").unwrap_or("".to_string());
        let template = MainTemplate {
            styles,
            user,
            policy_text
        };
        template.render().ok().unwrap_or_default()
    }
}
