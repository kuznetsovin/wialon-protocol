use tokio::net::{TcpListener,TcpStream};
use std::error::Error;
use tokio::io::{AsyncReadExt};

#[derive(Copy, Clone)]
struct Server {addr: &'static str}

impl Server {
    fn new(ip_addr: &'static str) -> Server {
        Server{ addr: ip_addr }
    }

    async fn conn_handler(self, mut socket: TcpStream) {
        let mut buf = [0; 1024];

        loop {
            match socket.read(&mut buf).await {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }

                    match parse_packet(&buf[0..n]){
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
    }

    async fn start_server(self) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(self.addr).await?;

        loop {
            let (socket, _) = listener.accept().await?;

            tokio::spawn(async move {
                self.conn_handler(socket).await
            });
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let s = Server::new("127.0.0.1:5555");

    s.start_server().await
}


fn parse_packet(b: &[u8]) -> Result<String, &str> {
    if b[0] == 0x23 && b[b.len()-2..] == [0x0D, 0x0A] {
        Ok(String::from_utf8(b.to_vec()).unwrap())
    } else {
        Err("Не корректное сообщение")
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn parsing_incorrect_msg() {
//         assert_eq!(Err("Не корректное сообщение"), parse_packet(&[0x77, 0x65, 0x72, 0x0a]));
//         assert_eq!(Err("Не корректное сообщение"), parse_packet(&[0x23, 0x77, 0x65, 0x72, 0x0a]));
//     }

//     #[test]
//     fn parsing_correct_msg() {
//         assert_eq!(Ok(String::from("#L#1;1\r\n")), parse_packet(&[0x23, 0x4c, 0x23, 0x31, 0x3b, 0x31, 0x0d, 0x0A]));
//     }
// }
