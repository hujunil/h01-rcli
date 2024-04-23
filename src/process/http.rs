use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
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
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("reading file: {:?}", p);
    match tokio::fs::read(p).await {
        Ok(content) => (
            StatusCode::OK,
            String::from_utf8_lossy(&content).to_string(),
        ),
        Err(_) => (StatusCode::NOT_FOUND, "File not found".to_string()),
    }
}
