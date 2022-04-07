
use ztop::startup::Application;
use ztop::configuration::get_configuration;
use ztop::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let subscriber = get_subscriber("ztop".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
