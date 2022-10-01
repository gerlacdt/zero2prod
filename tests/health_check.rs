use std::net::TcpListener;

const BIND_ADDR: &str = "127.0.0.1:3000";

#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/health_check", BIND_ADDR))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let expected_length = Some(10); // I am alive
    assert_eq!(expected_length, response.content_length());
}

fn spawn_app() {
    let listener = TcpListener::bind(BIND_ADDR).unwrap();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
