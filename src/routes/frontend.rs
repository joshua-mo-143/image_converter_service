use axum::response::Html;
use pulldown_cmark::{html::push_html, Parser};
use tokio::fs;

pub async fn serve_homepage() -> Html<String> {
    let file = fs::read_to_string("templates/homepage.md").await.unwrap();
    let parser = Parser::new(&file);

    let mut html_output = String::new();
    push_html(&mut html_output, parser);

    Html(html_output)
}
