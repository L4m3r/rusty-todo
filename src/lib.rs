pub mod config;
mod routes;
use axum::{routing::{get, post}, Router, serve::Serve};
use routes::{health_check, list_tasks};
use routes::create_task;
use sqlx::PgPool;
use tokio::net::TcpListener;

async fn root() -> &'static str {
    "Hello, world!"
}

pub fn run(listener: TcpListener, pool: PgPool) -> Serve<Router, Router> {
    let router = Router::new()
        .route("/", get(root))
        .route("/health_check", get(health_check))
        .route("/tasks", get(list_tasks))
        .route("/tasks", post(create_task))
        .with_state(pool);
    axum::serve(listener, router)
}
