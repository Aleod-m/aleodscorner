pub(crate) use axum::{response::IntoResponse, routing::get};
use templates::render_template;
use crate::template_responders;

use super::elements;


pub fn setup_routing(router: axum::Router) -> axum::Router {
    router
        .route("/home", get(home))
        .route("/home/header", get(header))
        .route("/home/content", get(content))
        .route("/home/footer", get(elements::none))
        .route("/home/aside", get(elements::none))
}

template_responders! {
    header => pages::header_html("home"),
    content => pages::home::content_html,
}

pub async fn home() -> impl IntoResponse {
    render_template!(
        utils::page_html,
        "Aleod's Web Corner.",
        "home",
        true,
        "home.css"
    )
}

