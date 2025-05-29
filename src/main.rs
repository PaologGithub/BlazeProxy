use std::sync::Arc;

use anyhow::Result;

use blaze_proxy::config::Config;
use blaze_proxy::proxy::run_proxy;

#[tokio::main]
async fn main() -> Result<()> {
    // Config
    let config: Config = Config::new("BlazeProxy.toml")?;
    let forward_to: Arc<String> = Arc::new(config.forward_to);

    run_proxy(config.local_bind, forward_to).await
}