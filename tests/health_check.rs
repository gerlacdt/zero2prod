use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};

const BIND_ADDR: &str = "127.0.0.1:0";

#[tokio::test]
async fn health_check_works() {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();
    let connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let random_address = spawn_app(connection);
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
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let connection2 = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let app_address = spawn_app(connection2);
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

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let configuration = get_configuration().expect("Faild to read configuration.");
    let connection_string = configuration.database.connection_string();
    let connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let app_address = spawn_app(connection);
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

fn spawn_app(connection: PgConnection) -> String {
    let listener = TcpListener::bind(BIND_ADDR).expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener, connection).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("127.0.0.1:{}", port)
}
