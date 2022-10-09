use crate::helpers::{clean_db, spawn_app};

#[tokio::test]
async fn health_check_works() {
    clean_db().await;
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    dbg!(format!("{}/health_check", &app.address));

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let expected_length = Some(10); // I am alive
    assert_eq!(expected_length, response.content_length());
}
