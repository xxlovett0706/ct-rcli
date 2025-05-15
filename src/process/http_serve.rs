use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", path, addr);

    let state = HttpServeState { path: path.clone() };

    // axum router
    let app = Router::new()
        .route("/file/{*path}", get(file_handler))
        .fallback_service(ServeDir::new(path))
        .with_state(Arc::new(state));

    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);

    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File {} not found", p.display()),
        )
    } else {
        // TODO: find p is directory, list all files in the directory
        if p.is_dir() {
            let mut entries = Vec::new();
            let mut dir = tokio::fs::read_dir(p).await.unwrap();
            while let Ok(Some(entry)) = dir.next_entry().await {
                entries.push(entry.file_name().to_string_lossy().to_string());
            }
            (StatusCode::OK, entries.join("\n"))
        } else {
            match tokio::fs::read_to_string(p).await {
                Ok(content) => {
                    info!("Read {} bytes", content.len());
                    (StatusCode::OK, content)
                }
                Err(e) => {
                    warn!("Error reading file {:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });

        let (status, content) = file_handler(State(state), Path("src".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        println!("{}", content);
        // assert!(content.contains("[dependencies]"));
    }
}
