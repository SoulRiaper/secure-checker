use web_view::*;

fn main() {
    web_view::builder()
        .title("Привет, мир!")
        .content(Content::Html("<h1>Hello, World!</h1>"))
        .size(320, 480)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
