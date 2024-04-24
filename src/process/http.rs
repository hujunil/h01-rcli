use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::Response,
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use tracing::info;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_server(dir: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let state = HttpServeState { path: dir.clone() };

    let router = Router::new()
        .nest_service("/tower", ServeDir::new(dir.clone()))
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> Response {
    let p = std::path::Path::new(&state.path).join(&path);
    info!("Request path: {:?}", p);
    if p.is_dir() {
        // handle directory
        let content = rea_dir_with_html(p, &path).await;
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html; charset=UTF-8")
            .body(Body::from(content))
            .unwrap()
    } else {
        let (code, body) = match tokio::fs::read(p).await {
            Ok(content) => (
                StatusCode::OK,
                String::from_utf8_lossy(&content).to_string(),
            ),
            Err(_) => (StatusCode::NOT_FOUND, "File not found".to_string()),
        };

        Response::builder()
            .status(code)
            .body(Body::from(body))
            .unwrap()
    }
}

async fn rea_dir_with_html(dir_path: PathBuf, path: &str) -> String {
    let mut dir = tokio::fs::read_dir(dir_path).await.unwrap();

    let mut files = Vec::new();

    while let Some(entry) = dir.next_entry().await.unwrap() {
        let sub_path = entry.path();
        if let Some(file_name) = sub_path.file_name().map(|s| s.to_string_lossy()) {
            let file_path = format!("/{}/{}", path, file_name);
            files.push(format!("<li><a href=\"{file_path}\">{file_name}</a></li>"));
        }
    }
    let html = format!(
        "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\" /></head><body><ul>{}</ul></body></html>",
        files.join("\n")
    );

    html
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_dir_handle() {
        let url_path = "fixtures";
        let path = std::path::Path::new("./").join(url_path);
        println!("{:?}", path);
        let resp = super::rea_dir_with_html(path, url_path).await;
        println!("{}", resp);
    }
}
