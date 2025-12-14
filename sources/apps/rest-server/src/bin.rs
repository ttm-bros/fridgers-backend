
async fn main() -> anyhow::Result<()> {
    use rest_server::config::Config;
    use rest_server::server::RestServer;
    use rest_server::tracing_util::init_tracing;

    // Initialize tracing for logging
    init_tracing("rest-server")?;

    // Load configuration from environment variables or config files
    let config = Config::load()?;

    // Create and start the REST server
    let server = RestServer::new(config);
    server.start().await?;

    Ok(())
}