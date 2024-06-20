use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

#[derive(serde::Serialize)]
pub struct Task {
    id: uuid::Uuid,
    description: String,
    is_completed: bool,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct NewTask {
    pub description: String,
    pub is_completed: Option<bool>,
}

pub async fn list_tasks(State(pool): State<PgPool>) -> (StatusCode, Json<Vec<Task>>) {
    let tasks = sqlx::query_as!(Task, "SELECT * from tasks")
        .fetch_all(&pool)
        .await
        .unwrap();
    (StatusCode::OK, Json(tasks))
}

pub async fn create_task(
    State(pool): State<PgPool>,
    Query(task): Query<NewTask>,
) -> (StatusCode, ()) {
    sqlx::query!(
        "INSERT INTO tasks(id, description, is_completed)
                 VALUES ($1, $2, $3)",
        uuid::Uuid::new_v4(),
        task.description,
        task.is_completed.unwrap_or_default()
    )
    .execute(&pool)
    .await
    .unwrap();
    (StatusCode::CREATED, ())
}
