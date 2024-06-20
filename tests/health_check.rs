mod helpers;

use axum::http::StatusCode;

#[tokio::test]
async fn health_check() {
    let app = helpers::spawn_app().await;

    let client = reqwest::Client::new();

    let responce = client
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .unwrap();

    assert_eq!(responce.status(), StatusCode::OK);

    let body = responce.bytes().await.unwrap();
    assert_eq!(body, "");
}
