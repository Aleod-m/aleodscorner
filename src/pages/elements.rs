use axum::{response::IntoResponse, extract::Query};
use axum_htmx::HxCurrentUrl;
use hyper::StatusCode;
use serde::Deserialize;
use templates::render_template;

pub async fn default_footer() -> impl IntoResponse {
    render_template!(utils::default_footer_html)
}

pub async fn header(HxCurrentUrl(url): HxCurrentUrl) -> impl IntoResponse {
    let _url = url
        .and_then(|url| url.path().split("/").last().map(String::from))
        .unwrap();
    println!("{_url}");
    render_template!(utils::default_header_html)
}

pub async fn no_nav(_: Query<SideBarState>) -> StatusCode {StatusCode::OK}

#[derive(Deserialize, Default)]
pub struct SideBarState {
    pub expanded: bool,
}
