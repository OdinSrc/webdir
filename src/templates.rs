use askama::Template;
use axum::body::BoxBody;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

#[derive(Template)]
#[template(path = "directory.html")]
pub struct DirectoryTemplate {
    pub current_path: String,
    pub files: Vec<String>,
}

impl IntoResponse for DirectoryTemplate {
    fn into_response(self) -> Response<BoxBody> {
        let t = self;
        match t.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "not_found.html")]
pub struct NotFoundTemplate {}

impl IntoResponse for NotFoundTemplate {
    fn into_response(self) -> Response<BoxBody> {
        let t = self;
        match t.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
