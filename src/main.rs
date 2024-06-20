use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = rusty_todo::config::get_config().unwrap();
    let connection_pool = PgPool::connect(&config.database.connection_string()).await.unwrap();

    tracing_subscriber::fmt::init();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    let app = rusty_todo::run(listener, connection_pool);
    tracing::info!("listening on 127.0.0.1:8080");
    app.await.unwrap();
    Ok(())
}
