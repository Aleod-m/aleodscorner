use axum::{response::IntoResponse, routing::get};
use maud::html;
use templates::render_template;

use crate::prelude::RouterExt;

pub fn setup_routing(router: axum::Router) -> axum::Router {
    router
        .route("/articles", get(articles))
        .route("/articles/content", get(content))
        .route("/articles/nav", get(nav))
        .redirect("/articles/footer", "/elem/default_footer")
}

pub async fn articles() -> impl IntoResponse {
    render_template!(
        utils::page_html,
        "Aleod's Web Corner.",
        "articles",
        Some("home"),
        true,
    )
}

async fn content() -> impl IntoResponse {
    html! {"content"}
}

async fn nav() -> impl IntoResponse {
    html! {"nav"}
}
