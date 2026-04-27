use axum::{
    Router,
    routing::get,
};
use tokio::net::TcpListener;
use sqlx::postgres::PgPool;
use dotenv::dotenv;

use production_api::http::health;
use production_api::http::prediction;

#[tokio::main]

async fn main(){


    //Postgres server config
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect("&database_url").await.unwrap()?;

    //routes of the API
    let router = axum::Router::new()
        .route("/health", axum::routing::get(health::hello_handler))
        .route("/predictions", get(prediction::get_predictions))
        .with_state(db);

    //TCP listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.unwrap();

    //Start HTTP server
    axum::serve(listener, router).await.unwrap();
}

