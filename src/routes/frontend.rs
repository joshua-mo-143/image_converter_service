use axum::response::Html;
use pulldown_cmark::{html::push_html, Parser};
use tokio::fs;
use std::path::PathBuf;
use std::str::FromStr;

pub async fn serve_homepage() -> Html<String> {
	let path = fs::canonicalize("templates/homepage.md").await.unwrap();
		
	println!("{:?}", path.display());
    let file = fs::read_to_string(path).await.unwrap();
    let parser = Parser::new(&file);

    let mut html_output = String::new();
    push_html(&mut html_output, parser);

    Html(html_output)
}
