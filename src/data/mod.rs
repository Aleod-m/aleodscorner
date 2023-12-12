use axum::{response::IntoResponse, routing::get, Router};
use maud::html;

use crate::utils;

async fn mail() -> impl IntoResponse {
    utils::mail("adriendml99@protonmail.com")
}

async fn phone() -> impl IntoResponse {
    utils::phone("+33617506114")
}

async fn github() -> impl IntoResponse {
    utils::link("https://www.github.com/AdrienDML", "github.com/AdrienDML")
}

pub fn setup_routing(router: Router) -> Router {
    let sub_router = Router::new()
        .route("/mail", get(mail))
        .route("/phone", get(phone))
        .route("/github", get(github));

    router.nest("/aleod", sub_router)
}
