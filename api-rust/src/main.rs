use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use axum::http::Method;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct Counter {
    count: i32,
}

pub async fn create_pool() -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Failed to create pool")
}

async fn get_counter(State(pool): State<PgPool>) -> Json<Counter> {
    let result = sqlx::query_as::<_, Counter>("SELECT count FROM counter WHERE id = 1")
        .fetch_optional(&pool)
        .await
        .unwrap();

    match result {
        Some(counter) => Json(counter),
        None => Json(Counter { count: 0 }),
    }
}

async fn inc_counter(State(pool): State<PgPool>) -> Json<Counter> {
    let counter = sqlx::query_as::<_, Counter>(
        "INSERT INTO counter (id, count) VALUES (1, 1) 
         ON CONFLICT (id) DO UPDATE SET count = counter.count + 1 
         RETURNING count",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(counter)
}

async fn reset_counter(State(pool): State<PgPool>) -> Json<Counter> {
    let counter = sqlx::query_as::<_, Counter>(
        "INSERT INTO counter (id, count) VALUES (1, 1) 
         ON CONFLICT (id) DO UPDATE SET count = 0 
         RETURNING count",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(counter)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = create_pool().await;
    
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS counter (
            id INT PRIMARY KEY,
            count INT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .unwrap();


    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:8080".parse().unwrap(),
            "http://127.0.0.1:8080".parse().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(|| async { "Hello from Rust 🦀" }))
        .route("/count", get(get_counter))
        .route("/inc", post(inc_counter))
        .route("/reset", post(reset_counter))
        .with_state(pool)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
