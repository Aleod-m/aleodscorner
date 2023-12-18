use axum::{response::IntoResponse, routing::get};
use templates::render_template;

use super::elements;

pub fn setup_routing(router: axum::Router) -> axum::Router {
    router
        .route("/projects", get(articles))
        .route("/projects/header", get(elements::header))
        .route("/projects/content", get(content))
        .route("/projects/footer", get(elements::default_footer))
        .route("/projects/nav", get(elements::no_nav))
}


pub async fn articles() -> impl IntoResponse {
    render_template!(
        utils::page_html,
        "Aleod's projects.",
        "projects",
        true,
        "page.css"
    )
}

async fn content() -> impl IntoResponse {
    render_template!(pages::home::projects_html)
}

