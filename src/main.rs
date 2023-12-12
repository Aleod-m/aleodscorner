use axum::Router;
use hyper::Request;
use std::{env, net};
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

mod pages;

pub mod prelude {
    use axum::{response::Redirect, routing::get, Router};

    pub trait RouterExt {
        fn apply(self, f: fn(Router) -> Router) -> Router;
        fn redirect(self, route: &'static str, target: &'static str) -> Router;
    }

    impl RouterExt for Router {
        fn apply(self, f: fn(Router) -> Router) -> Router {
            f(self)
        }

        fn redirect(self, route: &'static str, target: &'static str) -> Router {
            self.route(route, get(|| async { Redirect::permanent(target) }))
        }
    }
}

use prelude::RouterExt;

fn not_htmx_request<T>(req: &Request<T>) -> bool {
    !req.headers().contains_key("hx-request")
}

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    // Get port from environement.
    let port = match env::var("SERVER_PORT") {
        Ok(port) => port.parse().expect("Invalid port number."),
        _ => panic!("No port provided."),
    };

    // Setup bind addr
    let addr: net::IpAddr = match env::var("SERVER_LOCAL") {
        Ok(_) => "127.0.0.1".parse().unwrap(),
        _ => "0.0.0.0".parse().unwrap(),
    };

    let router = Router::new()
        .apply(pages::add_pages)
        //.apply(data::setup_routing)
        .nest_service("/static", ServeDir::new("static"))
        .layer(LiveReloadLayer::new().request_predicate(not_htmx_request));

    axum::Server::bind(&net::SocketAddr::new(addr, port))
        .serve(router.into_make_service())
        .await
}
