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

fn spawn_app() -> String {
    let listener = TcpListener::bind(BIND_ADDR).expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("127.0.0.1:{}", port)
}
