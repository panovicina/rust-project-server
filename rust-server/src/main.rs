use std::{
    collections::HashMap,
    net::TcpListener,
    sync::{mpsc, Arc, Mutex},
    thread,
};

mod lib;

use lib::{
    logpringer::{Log, LogPrinter},
    request::Request,
    response::{DataRespose, Response},
    streamreader::StreamReader,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();

    let main_storage: HashMap<String, String> = HashMap::new();
    let storage_reference = Arc::new(Mutex::new(main_storage));
    let (main_sender, receiver) = mpsc::channel();
    let _ = LogPrinter::spawn(receiver);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let ip = stream.peer_addr().unwrap();
        let sender = main_sender.clone();
        let storage = storage_reference.clone();

        sender
            .send(Log::new(ip, storage.lock().unwrap().len(), None))
            .unwrap();

        let mut stream_reader = StreamReader::new(stream);
        thread::spawn(move || loop {
            let request = match stream_reader.receieve() {
                Ok(request) => request,
                Err(err)
                    if err.kind() == std::io::ErrorKind::ConnectionAborted
                        || err.kind() == std::io::ErrorKind::ConnectionReset =>
                {
                    break
                }
                Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => continue,
                Err(err) => {
                    eprintln!("{}\n{}", err, err.kind());
                    break;
                }
            };

            sender
                .send(Log::new(
                    ip,
                    storage.lock().unwrap().len(),
                    Some(request.clone()),
                ))
                .unwrap();

            let _ = stream_reader.send(match request {
                Request::Store { key, hash } => {
                    storage.lock().unwrap().insert(key, hash);
                    Response::Success { data: None }
                }
                Request::Load { key } => match storage.lock().unwrap().get(&key) {
                    Some(hash) => Response::Success {
                        data: Some(DataRespose::new(key, hash.clone())),
                    },
                    None => Response::KeyNotFound,
                },
            });
        });
    }
}
