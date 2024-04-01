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
async fn main() -> Result<(), std::io::Error> {
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
    let tcp_listener = tokio::net::TcpListener::bind(net::SocketAddr::new(addr, port)).await?;

    let router = Router::new()
        .apply(pages::add_pages)
        //.apply(data::setup_routing)
        .nest_service("/static", ServeDir::new("static"));

    #[cfg(debug_assertions)]
    let router = router.layer(LiveReloadLayer::new().request_predicate(not_htmx_request));

    axum::serve(tcp_listener, router).await
}
