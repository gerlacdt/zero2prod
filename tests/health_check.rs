use std::net::TcpListener;

const BIND_ADDR: &str = "127.0.0.1:0";

#[tokio::test]
async fn health_check_works() {
    let random_address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/health_check", &random_address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let expected_length = Some(10); // I am alive
    assert_eq!(expected_length, response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("http://{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute POST request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("http://{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 BAD REQUEST when payload was {}",
            error_message
        );
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind(BIND_ADDR).expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("127.0.0.1:{}", port)
}
