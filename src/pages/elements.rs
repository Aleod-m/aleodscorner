use axum::response::IntoResponse;
use axum_htmx::HxCurrentUrl;
use serde_derive::Deserialize;
use templates::render_template;

pub async fn default_footer() -> impl IntoResponse {
    render_template!(utils::default_footer_html)
}

pub async fn header_nav(HxCurrentUrl(url): HxCurrentUrl) -> impl IntoResponse {
    let _url = url.and_then(|url| url.as_str().split("/").last().map(String::from)).unwrap();
    println!("{_url}");
    render_template!(utils::default_header_html, &_url)
}

#[derive(Deserialize)]
pub struct NavState {
    pub expended: bool,
}
