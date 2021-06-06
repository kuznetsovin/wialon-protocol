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

#[test]
fn test_login_packet_body() {
    use tokio::runtime::Runtime;
    use std::{thread, time};
    use std::io::prelude::*;
    use std::net::TcpStream;

    thread::spawn(move || {
        let rt = Runtime::new().unwrap();

        loop {
            rt.block_on(start_server("0.0.0.0:5555")).unwrap();
        }
    });
    // TODO: replace to channel
    thread::sleep(time::Duration::from_secs(1));

    let mut stream = TcpStream::connect("0.0.0.0:5555").unwrap();
    let rlt = &mut [0; 128];

    match stream.write(b"#L#1;1\r\n") {
        Ok(_) => {
            let sz = stream.read(rlt).unwrap();
            assert_eq!(&rlt[0..sz], b"#AL#1\r\n")
        },
        Err(e) => panic!("{}", e),
    };

    match stream.write(b"#SD#280421;055447;5355.09260;N;02732.40990;E;60;0;300;7\r\n") {
        Ok(_) => {
            let sz = stream.read(rlt).unwrap();
            assert_eq!(&rlt[0..sz], b"#ASD#1\r\n")
        },
        Err(e) => panic!("{}", e),
    };

    match stream.write(b"#D#280421;055500;5355.09260;N;02732.40990;E;60;0;300;7;22;5;5120;;eee;test1:1:1,var:2:4.5,texttest:3:1\r\n") {
        Ok(_) => {
            let sz = stream.read(rlt).unwrap();
            assert_eq!(&rlt[0..sz], b"#AD#1\r\n")
        },
        Err(e) => panic!("{}", e),
    };
}


