use super::encoder;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

pub struct RedisClient {
    tcp_stream: TcpStream,
}

impl RedisClient {
    pub fn new(uri: &str) -> io::Result<RedisClient> {
        let tcpclient = TcpStream::connect(uri)?;
        let client = RedisClient {
            tcp_stream: tcpclient,
        };
        Ok(client)
    }

    pub fn send_cmd(mut self, cmd: &Vec<&str>) -> io::Result<String> {
        let cmd_str = encoder::encode_command(cmd);
        self.tcp_stream.write_all(cmd_str.as_bytes())?;
        let mut result_buff = vec![0u8; 1024];
        self.tcp_stream.read(&mut result_buff)?;
        let s: String = result_buff
            .iter()
            .filter(|val| **val != 0)
            .map(|val| *val as char)
            .collect();
        Ok(s)
    }
}
