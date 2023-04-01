use std::fs;
use std::path::PathBuf;

use axum::body::{boxed, Body};
use axum::extract::State;
use axum::http::Request;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

use tower::util::ServiceExt;

use tower_http::services::ServeFile;

use crate::templates::{DirectoryTemplate, NotFoundTemplate};

#[derive(Clone)]
struct AppState {
    root_dir: String,
}

pub async fn serve(bind: &str, port: u16, root_dir: String) {
    let url = format!("{bind}:{port}");
    tracing::info!("Serving HTTP on {url}");

    let app = Router::new()
        .route("/dirservex", get(root))
        .fallback(all_route_handler)
        .with_state(AppState { root_dir });

    axum::Server::bind(&url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn all_route_handler(
    State(app_state): State<AppState>,
    req: Request<Body>,
) -> impl IntoResponse {
    let url_path = req.uri().path().to_owned();

    let mut local_path = PathBuf::new();
    local_path.push(&app_state.root_dir);
    local_path.push(url_path.trim_start_matches("/"));

    let current_path_str: String = local_path
        .to_str()
        .unwrap()
        .trim_start_matches(".")
        .to_string();

    println!("{:?}", current_path_str);

    match local_path.is_dir() {
        true => {
            // Handling directories
            let mut files: Vec<String> = Vec::new();
            for file in fs::read_dir(&local_path).unwrap() {
                files.push(file.unwrap().file_name().to_str().unwrap().to_string());
            }

            Ok(DirectoryTemplate {
                current_path: add_trailing_slash(&current_path_str),
                files,
            }
            .into_response())
        }
        false => {
            // Serving file type

            if local_path.exists() {
                return Ok(ServeFile::new(local_path)
                    .oneshot(Request::new(()))
                    .await
                    .map(|res| res.map(boxed))
                    .into_response());
            } else {
                return Err(NotFoundTemplate {}.into_response());
            }
        }
    }
}

fn add_trailing_slash(url: &str) -> String {
    if !url.ends_with('/') {
        return format!("{}{}", url, "/");
    }
    url.to_string()
}
