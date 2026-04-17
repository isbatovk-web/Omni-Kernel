use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use sysinfo::System;
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;
use crate::metrics::{MetricsEngine, MetricsReport};

#[derive(Clone)]
pub struct AppState {
    pub metrics_engine: Arc<Mutex<MetricsEngine>>,
    pub sysinfo: Arc<Mutex<System>>,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub metrics: MetricsReport,
    pub sys: SysReport,
    pub uptime_secs: u64,
}

#[derive(Serialize)]
pub struct SysReport {
    pub total_memory: u64,
    pub used_memory: u64,
    pub global_cpu_usage: f32,
    pub cpus: usize,
}

#[derive(Deserialize)]
pub struct CommandRequest {
    command: String,
}

pub async fn start_server(metrics: MetricsEngine) {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Verification step for distribution
    if !std::path::Path::new("dashboard").exists() {
        eprintln!("\n[ERROR] Dashboard folder not found!");
        eprintln!("Please ensure the 'dashboard' directory is in the same folder as the exe.");
        eprintln!("Download the full release at: https://omni-kernel.github.io\n");
        return;
    }

    let state = AppState {
        metrics_engine: Arc::new(Mutex::new(metrics)),
        sysinfo: Arc::new(Mutex::new(sys)),
    };

    let api_routes = Router::new()
        .route("/status", get(get_status))
        .route("/command", post(post_command));

    let app = Router::new()
        .nest("/api", api_routes)
        .fallback_service(ServeDir::new("dashboard"))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("  🚀 Dashboard running at http://localhost:3000");
    
    axum::serve(listener, app).await.unwrap();
}

async fn get_status(State(state): State<AppState>) -> Json<StatusResponse> {
    // Refresh sysinfo softly locally in this handler to not block heavily
    let mut sys = state.sysinfo.lock().unwrap();
    sys.refresh_cpu_usage();
    sys.refresh_memory();

    let sys_report = SysReport {
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        global_cpu_usage: sys.global_cpu_info().cpu_usage(),
        cpus: sys.cpus().len(),
    };

    let metrics = state.metrics_engine.lock().unwrap();
    let report = metrics.generate_report();
    let uptime = report.uptime.as_secs();

    Json(StatusResponse {
        metrics: report,
        sys: sys_report,
        uptime_secs: uptime,
    })
}

async fn post_command(Json(req): Json<CommandRequest>) -> String {
    // Simple mock terminal handler for the demo dashboard
    match req.command.trim() {
        "omni status" => "✓ System operational • Dashboard OK".to_string(),
        "mem info" => "✓ Shared Pool: 16MB | Free: 2.9MB".to_string(),
        "bridge stats" => "→ Total Calls: Analyzing...\n→ Latency avg: 42μs".to_string(),
        "module list" => "1. core_engine [Rust] \n2. ml_inference [Python] \n3. ui_renderer [JS]".to_string(),
        "ping" => "pong! Backend is reachable.".to_string(),
        cmd if cmd.starts_with("omni inject") => {
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            if parts.len() > 2 {
                format!("✓ SUCCESS: Module '{}' sandboxed and loaded into Omni-Kernel.", parts[2])
            } else {
                "Error: Please specify a module to inject.".to_string()
            }
        },
        cmd => format!("Command not found: {}", cmd),
    }
}
