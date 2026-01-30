use zero2prod::{
    configuration::get_configuration,
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};
use rustls::crypto::CryptoProvider;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = CryptoProvider::install_default(rustls::crypto::aws_lc_rs::default_provider());
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
