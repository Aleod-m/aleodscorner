include!(concat!(env!("OUT_DIR"), "/templates.rs"));

pub use templates::*;

pub struct RenderTemplate<T: FnOnce(&mut Vec<u8>) -> std::io::Result<()>>(pub T);

impl<T: FnOnce(&mut Vec<u8>) -> std::io::Result<()>> axum::response::IntoResponse
    for RenderTemplate<T>
{
    fn into_response(self) -> axum::response::Response {
        let mut buf = Vec::new();
        match self.0(&mut buf) {
            Ok(()) => axum::response::Html(buf).into_response(),
            Err(_e) => {
                // TODO: logging
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Render failed",
                )
                    .into_response()
            }
        }
    }
}

#[macro_export]
macro_rules! render_template {
    ($root:ident $(:: $sub:ident )*) => {{
        use $crate::RenderTemplate;
        RenderTemplate(|o| $crate::templates::$root$(::$sub)*(o))
        }};
    ($root:ident $(:: $sub:ident )*, $($arg:expr),* $(,)*) => {{
        use $crate::RenderTemplate;
        RenderTemplate(move |o| $crate::templates::$root$(::$sub)*(o, $($arg),*))
    }}
}
