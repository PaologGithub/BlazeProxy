use tokio::io::{copy_bidirectional, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use anyhow::Result;

#[deprecated(note = "Please use the 'BlazeProxy' struct instead of run_proxy")]
pub async fn run_proxy(local_bind: String, server: toml::Value) -> Result<()> {
    let ip: String = server["ip"]
        .as_str()
        .expect(&format!("Error: Server with data {} has not a string for 'ip' value.", server))
        .to_string();

    let listener: TcpListener = TcpListener::bind(&local_bind).await?;
    println!("Proxy listening on {}", &local_bind);
    println!("Proxy will redirect to {}", ip);

    loop {
        let (mut inbound, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        let ip: String = ip.clone();

        tokio::spawn(async move {
            match TcpStream::connect(ip).await {
                Ok(mut outbound) => {
                    println!("Connection to target server was successful.");

                    match copy_bidirectional(&mut inbound, &mut outbound).await {
                        Ok((from_client, from_server)) => {
                            println!(
                                "Connection closed. Sent {} bytes to server, received {} bytes.",
                                from_client, from_server
                            );
                        }

                        Err(e) => {
                            eprintln!("Relay error: {:?}", e);
                            eprintln!("Closing connection.");

                            let _ = inbound.shutdown().await;
                            let _ = outbound.shutdown().await;
                        }
                    }
                }

                Err(e) => {
                    eprintln!("Failed to connect to target server: {:?}", e);
                    eprintln!("Closing connection.");

                    let _ = inbound.shutdown().await;
                }
            }
        });
    }
}

pub struct BlazeProxy {
    ip: String,
    listener: TcpListener,
}

impl BlazeProxy {
    pub async fn new(local_bind: &str, server: toml::Value) -> Result<Self, String> {
        let ip: &str = server["ip"]
            .as_str()
            .ok_or("Missing or invalid 'ip' field in server")?; 
        
        let listener: TcpListener = TcpListener::bind(local_bind)
            .await
            .map_err(|err| format!("Failed to bind to {}: {}", local_bind, err))?;


        Ok(Self {
            ip: ip.to_string(),
            listener: listener,
        })
    }

    pub async fn run(&self) {
        loop {
            let (mut inbound, addr) = self.listener.accept()
                .await
                .expect("Error while accepting inbound and address");

            println!("New connection from {}", addr);

            let ip: String = self.ip.clone();

            tokio::spawn(async move {
                match TcpStream::connect(ip).await {
                    Ok(mut outbound) => {
                        println!("Connection to target server was successful.");

                        match copy_bidirectional(&mut inbound, &mut outbound).await {
                            Ok((from_client, from_server)) => {
                                println!(
                                    "Connection closed. Sent {} bytes to server, received {} bytes.",
                                    from_client, from_server
                                );
                            }

                            Err(e) => {
                                eprintln!("Relay error: {:?}", e);
                                eprintln!("Closing connection.");

                                let _ = inbound.shutdown().await;
                                let _ = outbound.shutdown().await;
                            }
                        }
                    }

                    Err(e) => {
                        eprintln!("Failed to connect to target server: {:?}", e);
                        eprintln!("Closing connection.");

                        let _ = inbound.shutdown().await;
                    }
                }
            });
        }
    }
}