use axum::{
    extract::{Query, Path},
    response::{IntoResponse, Response},
    routing::get,
    Form, Router,
};
use serde::Deserialize;
use templates::{render_template, Language, Colors};

use super::elements::{self, Expandable};

pub fn setup_routing(router: Router) -> Router {
    let router = router
        .route("/cv", get(page))
        .route("/cv/header", get(header))
        .route("/cv/content", get(content))
        .route("/cv/content/:lang", get(content))
        .route("/cv/aside", get(elements::none))
        .route("/cv/footer", get(elements::none));

    #[cfg(debug_assertions)]
    let router = router
        .route("/cv/dev/settings", get(dev_settings))
        .route("/cv/dev", get(dev_content).put(dev_content));

    router
}

async fn page() -> impl IntoResponse {
    render_template!(utils::page_html, "Curriculum Vitae", "cv", true, "cv.css")
}

async fn header() -> impl IntoResponse {
    render_template!(pages::cv::header_html)
}

#[derive(Deserialize)]
pub struct CVsettings {
    pub colors: String,
    pub language: String,
}

impl Default for CVsettings {
    fn default() -> Self {
        Self {
            colors: "dark".to_string(),
            language: "french".to_string(),
        }
    }
}

async fn dev_settings(expanded: Option<Query<Expandable>>) -> Response {
    if expanded.is_some() {
        render_template!(pages::cv::dev_settings_html).into_response()
    } else {
        elements::none().await.into_response()
    }
}

async fn content(language: Option<Path<String>>) -> impl IntoResponse {
    let language = language.map(|p| p.0).unwrap_or("english".to_string());
    let lang = if language == "english" {
        Language::English
    } else {
        Language::French
    };

    render_template!(pages::cv::content_html, lang)
}

async fn dev_content(settings: Option<Form<CVsettings>>) -> impl IntoResponse {
    let CVsettings {
        colors,
        language,
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

    render_template!(
        pages::cv::dev_content_html,
        lang,
        color,
    )
}
