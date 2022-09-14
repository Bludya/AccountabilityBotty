use AccountabilityBotty::configuration::get_configuration;
use AccountabilityBotty::startup::Application;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let mut application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}
