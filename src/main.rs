use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379")?;
    let cmd = "PING\r\n".as_bytes();
    stream.write_all(cmd)?;
    let mut result_buff = vec![0u8; 1024];
    stream.read(&mut result_buff)?;
    let s: String = result_buff.iter().map(|val| *val as char).collect();
    println!("{}", s);
    Ok(())
}
