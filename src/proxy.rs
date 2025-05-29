use tokio::io::copy_bidirectional;
use tokio::net::{TcpListener, TcpStream};
use std::sync::Arc;
use anyhow::Result;

pub async fn run_proxy(local_bind: String, forward_to: Arc<String>) -> Result<()> {
    let listener: TcpListener = TcpListener::bind(&local_bind).await?;
    println!("Proxy listening on {}", &local_bind);

    loop {
        let (mut inbound, addr) = listener.accept().await?;
        println! ("New connection from {}", addr);

        let forward_to = Arc::clone(&forward_to);

        tokio::spawn(async move {
            match TcpStream::connect(&*forward_to).await {
                Ok(mut outbound) => {
                    println!("Connected to target server: {}", forward_to);

                    match copy_bidirectional(&mut inbound, &mut outbound).await {
                        Ok((from_client, from_server)) => {
                            println!(
                                "Connection closed. Sent {} bytes to server, received {} bytes.",
                                from_client, from_server
                            );
                        }

                        Err(e) => eprintln!("Relay error: {:?}", e)
                    }
                }

                Err(e) => eprintln!("Failed to connect to target server: {:?}", e)
            }
        });
    }
}