use tokio::net::TcpListener;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::env;

mod wialon;

use log::{info, error};

async fn start_server(ip_addr: &str) -> Result<(), Box<dyn Error>> {
    info!("start receier {:?}", ip_addr);

    let listener = TcpListener::bind(ip_addr).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let packet = match socket.read(&mut buf).await {
                    Ok(n) => {
                        if n == 0 {
                            break;
                        }

                        match wialon::Packet::from(&buf[0..n]){
                            Ok(p) => p,
                            Err(err) =>{
                                error!("{:?}", err);
                                continue;
                            }                                 
                        }
                    },
                    Err(e) => {
                        error!("failed to read from socket: {:?}", e);
                        return;
                    }
                };

                info!("receiver packet: {:?}", packet);
                let resp = match packet.response(1){
                    Ok(p) => p,
                    Err(err) =>{
                        error!("{:?}", err);
                        continue;
                    }                                 
                };
                
                if packet.is_auth_packet() {
                    // TODO: auth process
                    info!("auth: {:?}", packet.get_auth_data());
                } else {
                    // TODO: create store interface
                    info!("position: {:?}", packet.get_navigate_data());
                }                

                if let Err(e) = socket.write_all(resp.to_string().as_bytes()).await {
                    error!("failed send ack: {:?}", e);
                    return;
                }
                info!("send ack: {:?}", resp);
            }
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    start_server("0.0.0.0:5555").await
}