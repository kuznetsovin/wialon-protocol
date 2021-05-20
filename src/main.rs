use tokio::net::TcpListener;
use std::error::Error;
use tokio::io::{AsyncReadExt};
mod wialon;

async fn start_server(ip_addr: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(ip_addr).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    Ok(n) => {
                        if n == 0 {
                            break;
                        }

                        match wialon::parse_packet(&buf[0..n]){
                            Ok(t) => println!("{}", t),
                            Err(err) => eprintln!("err: {:?}", err)
                        }
                    },
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
            }
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    start_server("127.0.0.1:5555").await
}