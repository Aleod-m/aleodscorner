use axum::{response::IntoResponse, routing::get, extract::Query};
use hyper::StatusCode;
use templates::render_template;

use super::elements::{self, NavState};

pub fn setup_routing(router: axum::Router) -> axum::Router {
    router
        .route("/articles", get(articles))
        .route("/articles/header", get(elements::header))
        .route("/articles/content", get(content))
        .route("/articles/footer", get(elements::default_footer))
        .route("/articles/nav", get(nav))
}

pub async fn nav(_: Query<NavState>) -> StatusCode {StatusCode::OK}

pub async fn articles() -> impl IntoResponse {
    render_template!(
        utils::page_html,
        "Aleod's articles.",
        "articles",
        true,
        "page.css"
    )
}

async fn content() -> impl IntoResponse {
    render_template!(pages::home::articles_html, vec![])
}

