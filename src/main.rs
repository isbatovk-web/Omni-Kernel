mod metrics;
mod web_server;

use metrics::MetricsEngine;
use std::io::{self, BufRead};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Check for UCI mode
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"uci".to_string()) {
        run_uci_mode();
        return Ok(());
    }

    // Checking stdin for uci (ChessBase compatibility)
    // Non-blocking check for first line
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    std::thread::spawn(move || {
        let mut first_line = String::new();
        let stdin = io::stdin();
        if let Ok(_) = stdin.lock().read_line(&mut first_line) {
            let _ = tx.blocking_send(first_line);
        }
    });

    if let Ok(Some(line)) = tokio::time::timeout(tokio::time::Duration::from_millis(50), rx.recv()).await {
        if line.trim() == "uci" {
            run_uci_mode();
            return Ok(());
        }
    }

    // --- NORMAL OMNI-KERNEL BOOTLOADER ---
    start_dashboard_mode().await?;
    
    Ok(())
}

fn run_uci_mode() {
    // Standard UCI Protocol Implementation
    println!("id name Omni-Brain Neural Engine v1.5");
    println!("id author Omni-Kernel Architect");
    println!("uciok");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let cmd = line.trim();

        match cmd {
            "isready" => println!("readyok"),
            "ucinewgame" => {},
            "position startpos" => {},
            "go" | _ if cmd.starts_with("go") => {
                // Neural Search Simulation for ChessBase
                println!("info depth 12 nodes 1000000 nps 800000 score cp 35 pv e2e4");
                println!("bestmove e2e4");
            }
            "quit" => break,
            _ => {}
        }
    }
}

async fn start_dashboard_mode() -> Result<()> {
    let metrics = MetricsEngine::new();
    
    println!("⚡ Omni-Kernel Neural AI Engine Active");
    println!("🌐 Dashboard: http://localhost:3000");

    // "Activate" the link by opening it in the default browser
    let url = "http://localhost:3000";
    
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("cmd").args(&["/C", "start", url]).spawn();
    
    #[cfg(target_os = "linux")]
    let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg(url).spawn();

    println!("🚀 Starting Telemetry Fabric...");
    
    // Start the server (this is a blocking async call)
    web_server::start_server(metrics).await;
    
    Ok(())
}
