use accountability_botty::configuration::get_configuration;
use accountability_botty::startup::Application;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let mut application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}
