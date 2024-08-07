use axum::{Router, routing::get, response::IntoResponse};
use templates::render_template;

use crate::template_responders;

use super::elements;


pub fn setup_routing(router: Router) -> Router {
    router
        .route("/articles", get(articles))
        .route("/articles/header", get(header))
        .route("/articles/content", get(content))
        .route("/articles/footer", get(elements::none))
        .route("/articles/nav", get(elements::none))
}

template_responders! {
    content => pages::home::articles_html,
    header => pages::header_html("articles")
}

pub async fn articles() -> impl IntoResponse {
    render_template!(
        utils::page_html,
        "Aleod's articles.",
        "articles",
        true,
        "home.css"
    )
}
