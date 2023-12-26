use std::env;
use std::error::Error;

use futures::FutureExt;
use tokio::io::copy_bidirectional;
use tokio::net::{TcpListener, TcpStream};

pub async fn prox() -> Result<(), Box<dyn Error>> {
    let listen_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8081".to_string());
    let server_addr = env::args()
        .nth(2)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    println!("Listening on: {}", listen_addr);
    println!("Proxying to: {}", server_addr);

    let listener = TcpListener::bind(&server_addr).await?;

    while let Ok((mut inbound, _)) = listener.accept().await {
        let mut outbound = TcpStream::connect(server_addr.clone()).await?;

        tokio::spawn(async move {
            copy_bidirectional(&mut inbound, &mut outbound)
                .map(|r| {
                    if let Err(e) = r {
                        eprintln!("Failed to proxy with error {}", e);
                    }
                })
                .await
        });
    }

    Ok(())
}
