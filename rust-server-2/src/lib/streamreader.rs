use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use super::{request::Request, response::Response};

pub struct StreamReader {
    stream: TcpStream,
}

impl StreamReader {
    pub fn new(stream: TcpStream) -> Self {
        StreamReader { stream }
    }

    pub fn receieve(&mut self) -> std::io::Result<Request> {
        let mut buffer = vec![];
        let mut reader = BufReader::new(self.stream.try_clone()?);
        reader.read_until(b'}', &mut buffer)?;
        let request = serde_json::from_slice(&buffer)?;
        Ok(request)
    }

    pub fn send(&mut self, respose: Response) -> std::io::Result<()> {
        self.stream
            .write_all(serde_json::to_string(&respose)?.as_bytes())
    }
}
