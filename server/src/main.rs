
mod app;
mod config;
mod database;
mod middleware;
mod modules;
mod shared;

use anyhow::Result;
use config::env::AppConfig;
#[tokio::main]
async fn main() -> Result<()> {
       let env_path = dotenvy::dotenv()?;
    println!("Loaded: {}", env_path.display());
    println!(
        "APP_NAME loaded: {}",
        std::env::var("APP_NAME").is_ok()
    );

    let config = AppConfig::load()?;

    println!("App: {}", config.app_name);
    println!("Environment: {}", config.app_env);
    println!("Host: {}", config.host);
    println!("Port: {}", config.port);
    println!("Log level: {}", config.log_level);
    println!("Log format: {}", config.log_format);
    let app = app::router::create_router();
    println!("App: {}", config.app_name);
    println!("Environment: {}", config.app_env);
    let listener =
        tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await?;

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
