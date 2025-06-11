use anyhow::Result;

use blaze_proxy::config::Config;
use blaze_proxy::proxy::BlazeProxy;

#[tokio::main]
async fn main() -> Result<()> {
    // Config
    let config: Config = Config::new("BlazeProxy.toml")?;

    // Default server
    let default_server: toml::Value = config.servers[&config.default_server].clone();

    // Proxy
    let proxy: BlazeProxy = BlazeProxy::new(
        &config.local_bind,
        default_server
    ).await
    .unwrap_or_else(
        |err| {panic!("Couldn't create proxy: {}", err)
    });

    // Run Procy
    proxy.run().await;

    Ok(())
}