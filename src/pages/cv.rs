use axum::{
    extract::Query,
    response::{IntoResponse, Response},
    routing::get,
    Form, Router,
};
use serde::Deserialize;
use templates::{render_template, Language, Colors};

use super::elements::{self, Expandable};

pub fn setup_routing(router: Router) -> Router {
    router
        .route("/cv", get(cv))
        .route("/cv/header", get(header))
        .route("/cv/content", get(default_content).put(content))
        .route("/cv/aside", get(elements::none))
        .route("/cv/footer", get(elements::none))
        .route("/cv/settings", get(settings))
}

async fn cv() -> impl IntoResponse {
    render_template!(utils::page_html, "Curriculum Vitae", "cv", true, "cv.css")
}

async fn header() -> impl IntoResponse {
    render_template!(pages::cv::header_html)
}

async fn settings(expanded: Option<Query<Expandable>>) -> Response {
    if expanded.is_some() {
        render_template!(pages::cv::settings_html).into_response()
    } else {
        elements::none().await.into_response()
    }
}

#[derive(Deserialize)]
pub struct CVsettings {
    pub colors: String,
    pub language: String,
    #[serde(rename = "type")]
    pub ty: String,
}

impl Default for CVsettings {
    fn default() -> Self {
        Self {
            colors: "dark".to_string(),
            language: "french".to_string(),
            ty: "general".to_string(),
        }
    }
}
async fn default_content() -> impl IntoResponse {
    let  CVsettings {
        colors,
        language,
        ty,
    } = CVsettings::default();
    let lang = if language == "english" {
        Language::English
    } else {
        Language::French
    };

    let color = match colors.as_str() {
        "print" => Colors::Print,
        "light" => Colors::Light,
        "dark" => Colors::Dark,
        _ => {
            eprint!("Invalid colors setting found!");
            Colors::Print
        }
    };
    dbg!(&color);
    dbg!(&language);
    dbg!(&ty);
    render_template!(
        pages::cv::page_content_html,
        lang,
        color,
        &ty,
    )
}


async fn content(settings: Option<Form<CVsettings>>) -> impl IntoResponse {
    let CVsettings {
        colors,
        language,
        ty,
    } = settings.unwrap_or_default().0;
    let lang = if language == "english" {
        Language::English
    } else {
        Language::French
    };

    let color = match colors.as_str() {
        "print" => Colors::Print,
        "light" => Colors::Light,
        "dark" => Colors::Dark,
        _ => {
            eprint!("Invalid colors setting found!");
            Colors::Print
        }
    };
    dbg!(&color);
    dbg!(&language);
    dbg!(&ty);
    render_template!(
        pages::cv::content_html,
        lang,
        color,
        &ty
    )
}
