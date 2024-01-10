use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        stream.write(input.as_bytes())?;

        let mut buffer = [0; 1024];
        let size = stream.read(&mut buffer)?;
        println!("Received {}", from_utf8(&buffer[0..size]).unwrap());
    }
}
