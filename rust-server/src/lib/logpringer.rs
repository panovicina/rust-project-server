use std::{
    net::SocketAddr,
    sync::mpsc,
    thread::{self, JoinHandle},
};

use super::request::Request;

pub struct Log {
    ip: SocketAddr,
    request: Option<Request>,
    storage_capacity: usize,
}

impl Log {
    pub fn new(ip: SocketAddr, storage_capacity: usize, request: Option<Request>) -> Self {
        Log {
            ip,
            request,
            storage_capacity,
        }
    }
}

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let time = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S:%3f");
        match &self.request {
            None => {
                write!(
                    f,
                    "{} [{}] Connection established. Storage size: {}",
                    self.ip, time, self.storage_capacity
                )
            }
            Some(request) => {
                match request {
                    Request::Store { key, hash } => {
                        write!(f, "{} [{}] Received request to write new value \"{hash}\" by key \"{key}\". Storage size: {}.", self.ip, time, self.storage_capacity)
                    }
                    Request::Load { key } => {
                        write!(f, "{} [{}] Received request to get value by key \"{key}\". Storage size: {}.", self.ip, time, self.storage_capacity)
                    }
                }
            }
        }
    }
}

pub struct LogPrinter {}

impl LogPrinter {
    pub fn spawn(receiver: mpsc::Receiver<Log>) -> JoinHandle<()> {
        thread::spawn(move || loop {
            let log = receiver.recv().unwrap();
            println!("{log}");
        })
    }
}
