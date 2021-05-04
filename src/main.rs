use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::thread;


fn main() -> Result<(), std::io::Error> {
    start_server("127.0.0.1:5555")
}

fn start_server(addr: &str) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(addr)?;

    for stream in listener.incoming() {
        thread::spawn(move || {
            handle_client(stream.unwrap());
        });
    };

    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    println!("new conn port {}", stream.peer_addr().unwrap());
    loop {
        let mut buf = [0; 1024];
        match stream.read(&mut buf) {
            Ok(n) => {
                 if n == 0 {
                    break;
                 }

                match parse_packet(&buf[0..n]){
                    Ok(t) => {
                        println!("{}", t);
                    }
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            }

            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}

fn parse_packet(b: &[u8]) -> Result<String, &str> {
    if b[0] == 0x23 && b[b.len()-2..] == [0x0D, 0x0A] {
        Ok(String::from_utf8(b.to_vec()).unwrap())
    } else {
        Err("Не корректное сообщение")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_incorrect_msg() {
        assert_eq!(Err("Не корректное сообщение"), parse_packet(&[0x77, 0x65, 0x72, 0x0a]));
        assert_eq!(Err("Не корректное сообщение"), parse_packet(&[0x23, 0x77, 0x65, 0x72, 0x0a]));
    }

    #[test]
    fn parsing_correct_msg() {
        assert_eq!(Ok(String::from("#L#1;1\r\n")), parse_packet(&[0x23, 0x4c, 0x23, 0x31, 0x3b, 0x31, 0x0d, 0x0A]));
    }
}
