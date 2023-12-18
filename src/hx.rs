use axum::{routing::get, Router};
use hyper::StatusCode;

pub fn add_routes(app: Router) -> axum::Router {
    app.route("/hx/delete", get(|| async { StatusCode::OK }))
}
