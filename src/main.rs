use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::thread;


fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:5555")?;

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
    if b[0] == 0x23 && b[b.len()-2..] == [0x0D, 0xA] {
        Ok(String::from_utf8(b.to_vec()).unwrap())
    } else {
        Err("Не корректное сообщение")
    }
}
