use anyhow::Result;
use log::{info, error};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use env_logger::init;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let listener = TcpListener::bind("0.0.0.0:30313").await?;

    let mut buf = [0; 1024];

    info!("start listening");
    loop {
        let (mut socket, _) = listener.accept().await?;
        
        info!("Connection opened");
        loop{ 
            match socket.read(&mut buf).await {
                Ok(0) => {
                    info!("Connection closed");
                    break;
                },
                Ok(n) => {
                    let mut reply = buf[0..n].to_vec();
                    reply.push('>' as u8);
                    if let Err(e) = socket.write_all(&reply).await {
                        error!("Failed to write to socket; err = {:?}", e);
                    } else {
                        info!("Send back \"{}\"", String::from_utf8(reply).unwrap_or("Found invalid UTF-8".into()));
                    }
                },
                Err(e) => {
                        error!("Failed to read to socket; err = {:?}", e);
                }
            }
        } 
    }

    Ok(())
}
