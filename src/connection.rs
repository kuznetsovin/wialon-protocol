use mio::{Interest, Registry, Token};
use mio::net::TcpStream;
use mio::event::Source;

use log::{info, error};
use std::io;
use std::sync::mpsc::SyncSender;
use crate::wialon;
use crate::store::GeoPacket;
us˚e crate::wialon::ResponsePacket;
use std::io::{Read, Write};

pub struct Connection {
    imei: Vec<u8>,
    socket: TcpStream,
    bus: SyncSender<GeoPacket>,
}

impl Source for Connection {
    fn register(&mut self, registry: &Registry, token: Token, interests: Interest)
                -> io::Result<()>
    {
        self.socket.register(registry, token, interests)
    }

    fn reregister(&mut self, registry: &Registry, token: Token, interests: Interest)
                  -> io::Result<()>
    {
        self.socket.reregister(registry, token, interests)
    }

    fn deregister(&mut self, registry: &Registry) -> io::Result<()> {
        self.socket.deregister(registry)
    }
}

impl Connection {
    pub fn new(c: TcpStream, bus: SyncSender<GeoPacket>) -> Connection {
        Connection {
            imei: vec![0, 100],
            socket: c,
            bus,
        }
    }
    pub fn get_message(&mut self) -> io::Result<bool> {
        let mut connection_closed = false;
        let mut read_bytes = 0;
        let mut buf = vec![0; 2048];
        loop {
            match self.socket.read(&mut buf) {
                Ok(0) => {
                    connection_closed = true;
                    break;
                }
                Ok(n) => {
                    read_bytes += n;
                    if read_bytes == buf.len() {
                        buf.resize(buf.len() + 1024, 0);
                    }
                }
                Err(err) => match err.kind() {
                    io::ErrorKind::WouldBlock => break,
                    io::ErrorKind::Interrupted => continue,
                    _ => return Err(err)
                }
            }
        }

        if read_bytes > 0 {
            match wialon::Packet::from(&buf[..read_bytes]) {
                Ok(p) => {
                    info!("receiver packet: {:?}", p);
                    if p.is_auth_packet() {
                        // TODO: auth process
                        let auth = p.get_auth_data().unwrap();
                        info!("auth: {:?}", auth);

                        self.imei = auth.imei.as_bytes().to_vec();
                    } else {
                        self.bus.send(GeoPacket::new(
                            self.imei.to_owned(),
                            p.get_navigate_data().unwrap(),
                        )).unwrap();
                    }

                    match p.response(1) {
                        Ok(r) => self.send_message(r)?,
                        Err(err) => error!("{:?}", err),
                    }
                }
                Err(err) => error!("{:?}", err),
            }
        }


        if connection_closed {
            return Ok(true);
        }

        Ok(false)
    }

    fn send_message(&mut self, msg: ResponsePacket) -> io::Result<()> {
        loop {
            match self.socket.write_all(msg.to_string().as_bytes()) {
                Ok(_) => {
                    return Ok(());
                }
                Err(err) => match err.kind() {
                    io::ErrorKind::WouldBlock => {}
                    io::ErrorKind::Interrupted => continue,
                    _ => return {
                        error!("failed send ack: {:?}", err);
                        Err(err)
                    }
                }
            }
        }
    }
}
