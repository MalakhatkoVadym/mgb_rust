use clap::Parser;

mod api;
mod config;
mod db;
mod logger;
mod state;

use api::{AppState, create_router};
use config::MGBConfig;
use db::Database;
use logger::init_logger;
use tracing::info;

#[derive(Parser)]
#[command(name = "mgb", version, about = "High-load gateway backend", long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long, default_value = "mgb.toml")]
    config_file_path: String,

    /// Enable debug mode (write logs to stdout)
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Load configuration from the specified file
    let config: MGBConfig = MGBConfig::parse_from_file(args.config_file_path.as_str());

    init_logger(args.verbose, config.log_file_path.clone(), config.log_level);

    info!("MGB app initialized");

    // Initialize database
    let db = match Database::new(&config.database_url).await {
        Ok(db) => {
            info!("Database connected successfully");
            db
        }
        Err(e) => {
            panic!("Failed to connect to database: {}", e);
        }
    };

    // Create application state
    let sm = state::StateMachine::new();

    // Log initial state in background
    let sm_clone = sm.clone();
    tokio::spawn(async move {
        let s = sm_clone.get_state().await;
        info!("Initial app state: {:?}", s);
    });

    let state = AppState { db, sm };

    // Build the router
    let app = create_router(state);

    // Start the server
    let addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("Failed to bind to {}: {}", addr, e));

    info!("Server listening on {}", addr);

    axum::serve(listener, app)
        .await
        .unwrap_or_else(|e| panic!("Server error: {}", e));
}
