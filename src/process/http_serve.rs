use anyhow::{Ok, Result};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tracing::info;
#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}
pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    info!("http_serve: {:?} {}", &path, addr);
    let state = HttpServeState { path };
    let router = Router::new()
        .route("/{*key}", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, router).await?;
    // TODO
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(key): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(state.path.as_path()).join(key);
    info!("Read file: {:?}", p);
    if p.exists() {
        let content = tokio::fs::read_to_string(p).await.unwrap();
        (StatusCode::OK, content)
    } else {
        (StatusCode::NOT_FOUND, format!("File not found: {:?}", p))
    }
}
