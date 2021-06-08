use std::net::SocketAddr;
use mio::net::{TcpListener, TcpStream};
use mio::event::Source;
use mio::{Events, Interest, Registry, Poll, Token};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::collections::VecDeque;
use std::env;

mod wialon;

use log::{info, error};


// Setup some tokens to allow us to identify which event is for which socket.
const SERVER: Token = Token(0);

struct Connection {
    socket: TcpStream,
    queue: VecDeque<wialon::ResponsePacket>,
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
    fn new(c: TcpStream) -> Connection {    
        Connection{
            socket: c,
            queue: VecDeque::new(),
        }
    }
    fn get_message(&mut self) -> io::Result<bool>{
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
            match wialon::Packet::from(&buf[..read_bytes]){
                Ok(p) => {
                    info!("receiver packet: {:?}", p);
                    if p.is_auth_packet() {
                        // TODO: auth process
                        info!("auth: {:?}", p.get_auth_data());
                    } else {
                        // TODO: create store interface
                        info!("position: {:?}", p.get_navigate_data());
                    }

                    match p.response(1) {
                        Ok(r) => self.queue.push_back(r),
                        Err(err) => error!("{:?}", err),
                    }                                                     
                },
                Err(err) => error!("{:?}", err),                      
            }            
        }
        

        if connection_closed {
            return Ok(true);
        }

        Ok(false)
    }

    fn send_message(&mut self) -> io::Result<bool>{
        let msg = match self.queue.pop_front(){
            Some(m) => m,
            _ => return Ok(false),
        }; 
        loop {
            match self.socket.write_all(msg.to_string().as_bytes()) {
                Ok(_) => {
                    return Ok(true)                                        
                },
                Err(err) => match err.kind() {
                    io::ErrorKind::WouldBlock => {},
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

struct Server {
    addr: SocketAddr,
    current_conn_token: Token,
    connections: HashMap<Token, Connection> 
}

impl Server {
    fn new(addr: &str) -> Server {
        Server{
            addr: addr.parse().unwrap(),
            current_conn_token: Token(SERVER.0 + 1),
            connections:  HashMap::new(),
        }
    }
    fn start(&mut self) -> io::Result<()> {
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

                        self.connections.insert(token, Connection::new(connection));
                    },
                    token => {
                        let connection = self.connections.get_mut(&token).unwrap(); 
                        if event.is_writable() {                            
                            connection.send_message()?;
                            poll.registry().reregister(connection, event.token(), Interest::READABLE)?;
                        }

                        if event.is_readable() {
                            let r = connection.get_message()?;
                            poll.registry().reregister(connection, event.token(), Interest::WRITABLE)?;
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

fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut s = Server::new("0.0.0.0:5555");
    s.start()
}

#[test]
fn test_server() {
    use std::{thread, time};
    use std::io::prelude::*;
    use std::net::TcpStream;

    let addr = "0.0.0.0:5555";
    thread::spawn(move || {
        let mut s = Server::new(addr);
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

    match stream.write(b"#ASD#1\n") {
        Ok(_) => {},
        Err(e) => panic!("{}", e),
    };
}


