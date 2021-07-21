use chrono::NaiveDateTime;
use std::io;
use std::env;
use std::str;
use serde::{Serialize};

mod wialon;
mod server;
mod connection;

use crate::wialon::ShortDataPacket;
use crate::server::Server;

#[derive(Serialize, Debug)]
pub struct GeoPacket {
    imei: String,
    timestamp: NaiveDateTime,
    lat: f64,
    lon: f64,
    speed: i16,
    course: i16,
    height: i16,
    sats: i16,
}

impl GeoPacket {
    fn new(client: Vec<u8>, data: &ShortDataPacket) -> GeoPacket {
        return GeoPacket {
            imei: String::from_utf8(client).unwrap(),
            timestamp: data.timestamp,
            lat: data.lat,
            lon: data.lon,
            speed: data.speed,
            course: data.course,
            height: data.height,
            sats: data.sats,
        };
    }
}

fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("usage: wialon-protocol <listner_addr> <buf_size>");
        return Ok(());
    }

    let addr: &str = &args[1];

    let buf_size: usize = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Required param buf size not found");
            return Ok(());
        }
    };

    let mut s = Server::new(addr, buf_size);
    s.start()
}

#[test]
fn test_server() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    use std::{thread, time};
    use std::io::prelude::*;
    use std::net::TcpStream;

    let addr = "0.0.0.0:5555";
    thread::spawn(move || {
        let mut s = Server::new(addr, 100);
        s.start()
    });
    // TODO: replace to channel
    thread::sleep(time::Duration::from_secs(1));

    let mut stream = TcpStream::connect(addr).unwrap();
    let rlt = &mut [0; 128];

    match stream.write(b"#L#1;1\r\n") {
        Ok(_) => {
            let sz = stream.read(rlt).unwrap();
            assert_eq!(&rlt[0..sz], b"#AL#1\r\n")
        }
        Err(e) => panic!("{}", e),
    };

    match stream.write(b"#SD#280421;055447;5355.09260;N;02732.40990;E;60;0;300;7\r\n") {
        Ok(_) => {
            let sz = stream.read(rlt).unwrap();
            assert_eq!(&rlt[0..sz], b"#ASD#1\r\n")
        }
        Err(e) => panic!("{}", e),
    };

    match stream.write(b"#D#280421;055500;5355.09260;N;02732.40990;E;60;0;300;7;22;5;5120;;eee;test1:1:1,var:2:4.5,texttest:3:1\r\n") {
        Ok(_) => {
            let sz = stream.read(rlt).unwrap();
            assert_eq!(&rlt[0..sz], b"#AD#1\r\n")
        }
        Err(e) => panic!("{}", e),
    };

    match stream.write(b"#ASD#1\n") {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    };
}


