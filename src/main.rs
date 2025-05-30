use anyhow::Result;

use blaze_proxy::config::Config;
use blaze_proxy::proxy::run_proxy;

#[tokio::main]
async fn main() -> Result<()> {
    // Config
    let config: Config = Config::new("BlazeProxy.toml")?;

    // Default server
    let default_server: toml::Value = config.servers[&config.default_server].clone();


    run_proxy(config.local_bind, default_server).await
}