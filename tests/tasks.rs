mod helpers;

use reqwest::StatusCode;

#[tokio::test]
async fn create_task() {
    let app = helpers::spawn_app().await;

    let client = reqwest::Client::new();

    let responce = client
        .post(format!(
            "{}/tasks?description=text&is_completed=true",
            &app.address
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(responce.status(), StatusCode::CREATED);

    let body = responce.bytes().await.unwrap();
    assert_eq!(body, "");

    let task = sqlx::query!("SELECT description, is_completed FROM tasks;")
        .fetch_one(&app.db_pool)
        .await
        .unwrap();

    assert_eq!(task.is_completed, true);
    assert_eq!(task.description, "text");
}

#[tokio::test]
async fn create_task_without_is_completed() {
    let app = helpers::spawn_app().await;

    let client = reqwest::Client::new();

    let responce = client
        .post(format!(
            "{}/tasks?description=tt",
            &app.address
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(responce.status(), StatusCode::CREATED);

    let body = responce.bytes().await.unwrap();
    assert_eq!(body, "");

    let task = sqlx::query!("SELECT description, is_completed FROM tasks;")
        .fetch_one(&app.db_pool)
        .await
        .unwrap();

    assert_eq!(task.is_completed, false);
    assert_eq!(task.description, "tt");
}
