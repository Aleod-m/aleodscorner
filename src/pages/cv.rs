use axum::{response::IntoResponse, routing::get, Router, extract::Query};
use serde_derive::Deserialize;
use templates::render_template;

use super::elements;

pub fn setup_routing(router: Router) -> Router {
    router
        .route("/cv", get(cv))
        .route("/cv/nav", get(nav))
        .route("/cv/page_nav", get(page_nav))
        .route("/cv/content", get(content))
        .route("/cv/footer", get(elements::default_footer))
}

async fn cv() -> impl IntoResponse {
    render_template!(
        utils::page_html,
        "Curriculum Vitae",
        "cv",
        Some("cv"),
        true
    )
}

async fn nav() -> impl IntoResponse {
    render_template!(pages::cv::nav_html)
}

#[derive(Deserialize)]
pub struct NavState {
    expended: bool,
}

async fn page_nav(
    Query(NavState { expended }): Query<NavState>
) -> impl IntoResponse {
    render_template!(pages::cv::page_nav_html, expended)
}

async fn content() -> impl IntoResponse {
    render_template!(pages::cv::content_html)
}
