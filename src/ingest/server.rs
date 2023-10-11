use std::fmt::{Debug};
use std::net::UdpSocket;

#[derive(Debug)]
struct BindError;

pub struct Server {
    socket: UdpSocket,
    buffer_size: i32
}

impl Server {
    pub fn bind(bind_addr: &str, buffer_size: i32) -> Server {
        let socket = UdpSocket::bind(bind_addr).unwrap();
        return Server{socket, buffer_size}
    }

    pub fn start(&self) {
        loop {
            let mut buf = [0; 1024];
            let (amt, src) = self.socket.recv_from(&mut buf).unwrap();
            println!("{:#?}", buf);
            self.socket.send_to(&[], src).unwrap();
        }
    }
}