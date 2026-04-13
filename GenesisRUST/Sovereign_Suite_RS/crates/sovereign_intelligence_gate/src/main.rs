use axum::{
    routing::{get, post},
    Router, Json, extract::State,
    http::{StatusCode, Method},
    response::IntoResponse,
};
use tower_http::cors::{CorsLayer, Any};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, sync::Arc};
use walkdir::WalkDir;
use anyhow::Result;

/// SOVEREIGN INTELLIGENCE GATE v10.0
/// Port: 8081 | Dynamic Nexus Anchor | Zero Hardcoded Paths
/// Axiom: 1.09277703703 Hz

// â”€â”€ Nexus Discovery â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

fn find_nexus_root() -> Option<PathBuf> {
    // Walk upward from current dir
    let mut curr = std::env::current_dir().ok()?;
    loop {
        if curr.join("sovereign.nexus").exists() {
            return Some(curr);
        }
        if !curr.pop() { break; }
    }
    // Fallback: GenesisOS_Core
    let fallback = PathBuf::from(r"C:\GENESIS\GenesisRUST\Sovereign_Suite_RS");
    if fallback.join("sovereign.nexus").exists() {
        return Some(fallback);
    }
    None
}

// â”€â”€ Shared State â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

#[derive(Clone)]
struct GateState {
    nexus_root: Arc<PathBuf>,
}

// â”€â”€ API Types â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

#[derive(Serialize)]
struct FsItem {
    name: String,
    path: String,
    is_dir: bool,
    size: u64,
}

#[derive(Deserialize)]
struct PathRequest {
    path: Option<String>,
}

#[derive(Deserialize)]
struct WriteRequest {
    path: String,
    content: String,
}

#[derive(Serialize)]
struct ConfigResponse {
    nexus_root: String,
    port: u16,
    resonance: f64,
}

// â”€â”€ Handlers â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

async fn handle_config(State(state): State<GateState>) -> impl IntoResponse {
    Json(ConfigResponse {
        nexus_root: state.nexus_root.to_string_lossy().to_string(),
        port: 8081,
        resonance: 1.09277703703,
    })
}

async fn handle_consensus(State(state): State<GateState>) -> impl IntoResponse {
    let directive_path = state.nexus_root.join("evolution_directive.json");
    if let Ok(content) = fs::read_to_string(directive_path) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            return Json(json).into_response();
        }
    }
    (StatusCode::NOT_FOUND, "No active consensus").into_response()
}

async fn handle_list(
    State(state): State<GateState>,
    Json(req): Json<PathRequest>,
) -> impl IntoResponse {
    let target = req.path
        .map(PathBuf::from)
        .unwrap_or_else(|| (*state.nexus_root).clone());

    if !target.exists() || !target.is_dir() {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Path not found or not a directory", "path": target.display().to_string() })),
        ).into_response();
    }

    let mut items: Vec<FsItem> = WalkDir::new(&target)
        .max_depth(1)
        .min_depth(1)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            name != ".git" && name != "target" && !name.starts_with('.')
        })
        .filter_map(|e| e.ok())
        .map(|e| {
            let meta = e.metadata().unwrap();
            FsItem {
                name: e.file_name().to_string_lossy().to_string(),
                path: e.path().display().to_string(),
                is_dir: meta.is_dir(),
                size: if meta.is_file() { meta.len() } else { 0 },
            }
        })
        .collect();

    // Directories first, then files â€” alphabetical
    items.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then(a.name.cmp(&b.name)));

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "items": items,
            "current_path": target.display().to_string()
        })),
    ).into_response()
}

async fn handle_read(
    State(_state): State<GateState>,
    Json(req): Json<PathRequest>,
) -> impl IntoResponse {
    let path = match req.path {
        Some(p) => PathBuf::from(p),
        None => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "path required" }))).into_response(),
    };

    match fs::read_to_string(&path) {
        Ok(content) => (StatusCode::OK, Json(serde_json::json!({ "content": content, "path": path.display().to_string() }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response(),
    }
}

async fn handle_write(
    State(_state): State<GateState>,
    Json(req): Json<WriteRequest>,
) -> impl IntoResponse {
    let path = PathBuf::from(&req.path);
    if let Some(parent) = path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response();
        }
    }
    match fs::write(&path, &req.content) {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({ "status": "written", "path": req.path }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response(),
    }
}

// â”€â”€ Entry Point â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

#[tokio::main]
async fn main() -> Result<()> {
    println!("\x1b[93m============================================================\x1b[0m");
    println!("\x1b[93m  SOVEREIGN INTELLIGENCE GATE v10.0 [IGNITING]             \x1b[0m");
    println!("\x1b[93m  Port: 8080 | Axiom: 1.09277703703 Hz                    \x1b[0m");
    println!("\x1b[93m============================================================\x1b[0m");

    let nexus_root = find_nexus_root()
        .ok_or_else(|| anyhow::anyhow!("Substrate adrift â€” no sovereign.nexus found"))?;

    println!("\x1b[92m[Nexus]\x1b[0m Anchored at: {:?}", nexus_root);

    let state = GateState {
        nexus_root: Arc::new(nexus_root),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/config",      get(handle_config))
        .route("/api/consensus",   get(handle_consensus))
        .route("/api/fs/list",     post(handle_list))
        .route("/api/fs/read",     post(handle_read))
        .route("/api/fs/write",    post(handle_write))
        .layer(cors)
        .with_state(state);

    let addr = "0.0.0.0:8081";
    println!("\x1b[92m[Gate]\x1b[0m Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
