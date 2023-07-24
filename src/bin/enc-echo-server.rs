use encrypt_stream::encstream::{DecStream, EncStream};
use futures::FutureExt;
use std::env;
use std::error::Error;
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listen_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8081".to_string());

    println!("Listening on: {}", listen_addr);

    let listener = TcpListener::bind(listen_addr).await?;

    while let Ok((inbound, _)) = listener.accept().await {
        let transfer = transfer(inbound).map(|r| {
            if let Err(e) = r {
                println!("Failed to transfer; error={}", e);
            }
        });

        tokio::spawn(transfer);
    }

    Ok(())
}

async fn transfer(mut inbound: TcpStream) -> Result<(), Box<dyn Error>> {
    let (ri, wi) = inbound.split();

    let key = [0x42; 32];
    let nonce = [0x24; 12];
    let mut ri = DecStream::new(ri, &key, &nonce);
    let mut wi = EncStream::new(wi, &key, &nonce);

    io::copy(&mut ri, &mut wi).await?;
    wi.shutdown().await?;

    Ok(())
}
