use std::net::SocketAddr;
use mio::{Events, Interest, Poll, Token};
use mio::net::TcpListener;
use std::io;
use std::thread;
use std::collections::HashMap;
use std::sync::mpsc::{sync_channel, SyncSender};

use log::info;
use crate::connection::Connection;
use crate::GeoPacket;

// mod connection;
// Setup some tokens to allow us to identify which event is for which socket.
const SERVER: Token = Token(0);

pub struct Server {
    addr: SocketAddr,
    current_conn_token: Token,
    connections: HashMap<Token, Connection>,
    bus: SyncSender<GeoPacket>,
}

impl Server {
    pub fn new(addr: &str, buf_size: usize) -> Server {
        let (sender, receiver) = sync_channel::<GeoPacket>(buf_size);

        thread::spawn(move || {
            loop {
                let p = receiver.recv().unwrap();
                let packet_json = serde_json::to_string(&p).unwrap();
                println!("{:?}", packet_json);
            }
        });

        Server {
            addr: addr.parse().unwrap(),
            current_conn_token: Token(SERVER.0 + 1),
            connections: HashMap::new(),
            bus: sender,
        }
    }
    pub fn start(&mut self) -> io::Result<()> {
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(128);
        let mut server = TcpListener::bind(self.addr)?;

        poll.registry().register(&mut server, SERVER, Interest::READABLE)?;

        info!("Start server: {}", self.addr);
        loop {
            poll.poll(&mut events, None)?;

            for event in events.iter() {
                match event.token() {
                    SERVER => loop {
                        let (mut connection, address) = match server.accept() {
                            Ok((connection, address)) => (connection, address),
                            Err(e) => match e.kind() {
                                io::ErrorKind::WouldBlock => break,
                                _ => return Err(e),
                            }
                        };

                        info!("Accepted connection from: {}", address);

                        let token = self.next_token();
                        poll.registry().register(&mut connection, token, Interest::READABLE)?;

                        self.connections.insert(token, Connection::new(connection, self.bus.to_owned()));
                    },
                    token => {
                        let connection = self.connections.get_mut(&token).unwrap();
                        if event.is_readable() {
                            let r = connection.get_message()?;
                            if r {
                                info!("Connection closed");
                                self.connections.remove(&token);
                            }
                        }
                    }
                }
            }
        }
    }

    fn next_token(&mut self) -> Token {
        self.current_conn_token.0 += 1;
        self.current_conn_token
    }
}
