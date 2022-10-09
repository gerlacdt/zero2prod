use zero2prod::configuration::get_configuration;
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_json_subscriber, get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_tracing();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}

fn init_tracing() {
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber("info".into());
        init_subscriber(subscriber);
    } else {
        let subscriber = get_json_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
        init_subscriber(subscriber);
    }
}
