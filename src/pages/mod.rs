use crate::prelude::*;
use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Router};
use axum_htmx::{HxEvent, HxLocation, HxResponseTrigger, TriggerMode};
use hyper::Uri;
use serde_derive::Deserialize;
use templates::render_template;

pub mod articles;
mod cv;
pub mod elements;
pub mod home;

pub type Failable<T> = Result<T, StatusCode>;

pub fn add_pages(app: Router) -> axum::Router {
    app.redirect("/", "/home")
        .route("/navigate", get(navigate_route))
        .apply(home::setup_routing)
        .apply(articles::setup_routing)
        .apply(cv::setup_routing)
        .fallback(handler_404)
}

async fn handler_404() -> impl IntoResponse {
    error_response(StatusCode::NOT_FOUND, "Page not found.")
}

pub fn error_response(status_code: StatusCode, msg: &str) -> impl IntoResponse + '_ {
    navigate((status_code, render_template!(error_html, status_code, msg)))
}

fn navigate(content: impl IntoResponse) -> impl IntoResponse {
    let trigger = HxResponseTrigger {
        events: [HxEvent {
            name: "navigate".into(),
        }]
        .to_vec(),
        mode: TriggerMode::Normal,
    };
    (trigger, content)
}

#[derive(Deserialize)]
pub struct Route {
    route: String,
}

async fn navigate_route(Query(Route { route }): Query<Route>) -> Failable<impl IntoResponse> {
    let events = [HxEvent {
        name: "navigate".to_string(),
    }]
    .to_vec();
    let Ok(uri) = route.parse::<Uri>() else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok((
        HxResponseTrigger {
            events,
            mode: TriggerMode::Normal,
        },
        HxLocation { uri },
        StatusCode::OK,
    ))
}
