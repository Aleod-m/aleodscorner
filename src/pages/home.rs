use axum::{extract::Query, response::IntoResponse, routing::get};
use templates::render_template;

use super::elements::{self, NavState};

pub fn setup_routing(router: axum::Router) -> axum::Router {
    router
        .route("/home", get(home))
        .route("/home/header", get(elements::header))
        .route("/home/content", get(content))
        .route("/home/footer", get(elements::default_footer))
        .route("/home/nav", get(page_nav))
        .route("/home/articles", get(articles))
}

pub async fn home() -> impl IntoResponse {
    render_template!(
        utils::page_html,
        "Aleod's Web Corner.",
        "home",
        true,
        "page.css"
    )
}

pub async fn content() -> impl IntoResponse {
    render_template!(pages::home::content_html)
}


pub async fn articles() -> impl IntoResponse {
    let articles = vec![data::ArticleSummary {
        title: "Who am i?".to_string(),
        summary: "Quick Presentation of myself".to_string(),
        id: 0,
    }];
    render_template!(pages::home::articles_html, articles)
}

pub async fn page_nav(Query(NavState { expended }): Query<NavState>) -> impl IntoResponse {
    render_template!(pages::home::page_nav_html, expended)
}
