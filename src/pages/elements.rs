use hyper::StatusCode;
use serde::Deserialize;

pub async fn none() -> StatusCode {StatusCode::OK}

#[macro_export]
macro_rules! template_responders {
    ($($v:vis $name:ident => $root:ident $(:: $sub:ident )* $(($($arg:expr),*))?),*$(,)?) => {
        $(
            $v async fn $name() -> impl IntoResponse {
                
                templates::RenderTemplate(|o| templates::$root$(::$sub)*(o $(,$($arg),*)?))
            }
        )*
    }
}

#[derive(Deserialize, Default)]
pub struct Expandable {
    pub expanded: bool,
}
