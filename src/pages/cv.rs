use axum::{extract::Query, response::IntoResponse, routing::{get, put}, Router, Form};
use serde::Deserialize;
use templates::render_template;

use super::elements::{self, SideBarState};

pub fn setup_routing(router: Router) -> Router {
    router
        .route("/cv", get(cv))
        .route("/cv/header", get(header))
        .route("/cv/footer", get(elements::default_footer))
        .route("/cv/content", get(content))
        .route("/cv/content", put(content))
        .route("/cv/aside", get(sidebar))
}

async fn cv() -> impl IntoResponse {
    render_template!(utils::page_html, "Curriculum Vitae", "cv", true, "cv.css")
}

async fn header() -> impl IntoResponse {
    render_template!(pages::cv::header_html)
}

async fn sidebar(state: Option<Query<SideBarState>>) -> impl IntoResponse {
    render_template!(pages::cv::sidebar_html, state.unwrap_or_default().expanded)
}

#[derive(Deserialize)]
pub struct CVsettings {
    pub colors: Option<String>,
    #[serde(rename = "type")]
    pub ty: String,
    pub layout: String,
}

impl Default for CVsettings {
    fn default() -> Self {
        Self { colors: None, ty: "general".to_string(), layout: "portrait".to_string()} 
    }
}

async fn content(settings: Option<Form<CVsettings>> ) -> impl IntoResponse {
    let CVsettings {
        colors,
        ty,
        layout,
    } = settings.unwrap_or_default().0;
    println!("{ty}");
    render_template!(pages::cv::content_html, colors.is_some(), &layout, &ty)
}
