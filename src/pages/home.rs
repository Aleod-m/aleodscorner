use axum::{response::IntoResponse, routing::get, extract::Query};
use templates::render_template;

use super::elements::{self, NavState};

pub fn setup_routing(router: axum::Router) -> axum::Router {
    router
        .route("/home", get(home))
        .route("/home/nav", get(elements::header_nav))
        .route("/home/content", get(content))
        .route("/home/page_nav", get(page_nav))
        .route("/home/footer", get(elements::default_footer))
        .route("/home/style", get(elements::default_footer))
}

pub async fn home() -> impl IntoResponse {
    render_template!(
        utils::page_html,
        "Aleod's Web Corner.",
        "home",
        Some("home"),
        true,
    )
}

pub async fn content() -> impl IntoResponse {
    render_template!(pages::home::content_html)
}

pub async fn page_nav(Query(NavState {expended}) : Query<NavState>) -> impl IntoResponse {
    render_template!(pages::home::page_nav_html, expended)
}
