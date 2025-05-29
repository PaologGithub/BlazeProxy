use std::sync::Arc;

use anyhow::Result;

use blaze_proxy::config::Config;
use blaze_proxy::proxy::run_proxy;

#[tokio::main]
async fn main() -> Result<()> {
    // Config
    let config: Config = Config::new("BlazeProxy.toml")?;

    // IP
    let ip: &str = config.servers[&config.default_server]["ip"]
                    .as_str()
                    .expect(&format!("The IP value of server {} is not a string.", &config.default_server));
                
    let forward_to: Arc<String> = Arc::new(ip.to_string());

    run_proxy(config.local_bind, forward_to).await
}