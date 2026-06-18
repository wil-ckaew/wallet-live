// src/main.rs
mod app;
mod auth;
mod error;
mod models;
mod repository;
mod routes;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    dotenvy::dotenv().ok();
    app::App::start().await
}