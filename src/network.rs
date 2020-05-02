use std::io::prelude::*;
use std::net::TcpStream;
use std::io::Error;

pub struct ServerConnection {
    stream: TcpStream
}

impl ServerConnection {
    pub fn connect(address: &str, port: u32) -> Result<ServerConnection, Error> {
        let server = format!("{}:{}", address, port);
        let stream: TcpStream = TcpStream::connect(&server)
            .expect(&format!("Failed to connect to `{}`", server));
        println!("Successfully connected to {}", server);
        Ok(ServerConnection {
            stream
        })
    }

    pub fn write_int(&mut self, n: u32) {
        self.stream.write(&n.to_be_bytes()).unwrap();
    }

    pub fn write_string(&mut self, s: &String) {
        self.write_int(s.len() as u32);
        self.stream.write(s.as_bytes()).unwrap();
    }

    pub fn read_int(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        self.stream.read_exact(&mut buf).unwrap();
        u32::from_be_bytes(buf)
    }

    pub fn read_string(&mut self) -> String {
        let len = self.read_int();
        let mut data =  vec![0u8; len as usize];
        self.stream.read(data.as_mut_slice()).unwrap();
        String::from_utf8(data).unwrap()
    }
}
