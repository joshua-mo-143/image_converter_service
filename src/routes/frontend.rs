use axum::{response::Html};
use pulldown_cmark::{html::push_html, Parser};

pub async fn serve_homepage() -> Html<String> {
	let parser = Parser::new(text_homepage());
	
	let mut html_output = String::new();
	push_html(&mut html_output, parser);
	
	Html(html_output)
}
pub fn text_homepage() -> &'static str {
	r#"# Webpnator - A self hosted solution for turning images and tarballs of images to .webp format. 
	
	<form method="POST" action="/convert">
	<label for="file">
	<span>## Upload a file:</span>
	<input type="file" name="file"></input>
	</label>
	<button type="submit">Upload</button>
	</form>
"#
}
