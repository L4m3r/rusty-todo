use std::future::IntoFuture;

use rusty_todo::{
    config::{get_config, DatabaseSettings},
    run,
};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use tokio::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub db_pool: PgPool,
    pub address: String,
}

pub async fn spawn_app() -> TestApp {
    let mut config = get_config().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    config.database.database_name = Uuid::new_v4().to_string();
    let pool = setup_database(&config.database).await;

    let app = run(listener, pool.clone());
    tokio::spawn(app.into_future());
    TestApp {
        db_pool: pool,
        address,
    }
}

async fn setup_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .unwrap();
    connection
        .execute(&*format!("CREATE DATABASE \"{}\";", config.database_name))
        .await
        .unwrap();

    let pool = PgPool::connect(&config.connection_string()).await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}
