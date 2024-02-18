use axum::{response::IntoResponse, routing::get};
use templates::render_template;

use super::{elements, home};

pub fn setup_routing(router: axum::Router) -> axum::Router {
    router
        .route("/projects", get(projects))
        .route("/projects/header", get(home::header))
        .route("/projects/content", get(content))
        .route("/projects/footer", get(elements::none))
        .route("/projects/nav", get(elements::none))
}


pub async fn projects() -> impl IntoResponse {
    render_template!(
        utils::page_html,
        "Aleod's projects.",
        "projects",
        true,
        "home.css"
    )
}

async fn content() -> impl IntoResponse {
    render_template!(pages::home::projects_html)
}

