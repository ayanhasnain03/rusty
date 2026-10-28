
mod app;
mod config;
mod database;
mod middleware;
mod modules;
mod shared;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let app = app::router::create_router();

    let listener =
        tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await?;

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
