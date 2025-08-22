#![allow(unused)]

mod handlers;

use axum::Router;
use axum::extract::State;
use axum::routing::{get, post};
use dotenv::dotenv;
use tokio::net::TcpListener;

use handlers::*;
use rushortner::initialize_database;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = initialize_database().await;

    let routes = Router::new()
        .route("/", get(home))
        .route("/api/shorten", post(shorten_url))
        .route("/{code}", get(redirect_to_full_url))
        .with_state(db);

    let listener = TcpListener::bind("0.0.0.0:8081").await.unwrap();
    axum::serve(listener, routes).await.unwrap();
}
