use axum::{extract::Query, response::IntoResponse, routing::get, Router};
use templates::render_template;

use super::elements::{self, NavState};

pub fn setup_routing(router: Router) -> Router {
    router
        .route("/cv", get(cv))
        .route("/cv/header", get(header))
        .route("/cv/footer", get(elements::default_footer))
        .route("/cv/content", get(content))
        .route("/cv/nav", get(page_nav))
}

async fn cv() -> impl IntoResponse {
    render_template!(utils::page_html, "Curriculum Vitae", "cv", true, "cv.css")
}

async fn header() -> impl IntoResponse {
    render_template!(pages::cv::header_html)
}

async fn page_nav(Query(NavState { expended }): Query<NavState>) -> impl IntoResponse {
    render_template!(pages::cv::page_nav_html, expended)
}

async fn content() -> impl IntoResponse {
    render_template!(pages::cv::content_html)
}
