use crate::blockchain::Blockchain;
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

pub struct Node {
    pub blockchain: Arc<Mutex<Blockchain>>,
}

impl Node {
    pub fn new() -> Self {
        let blockchain = Arc::new(Mutex::new(Blockchain::new()));
        Node { blockchain }
    }

    pub fn start(&self, address: &str) {
        let listener = TcpListener::bind(address).unwrap();
        println!("Node listening on {}", address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let blockchain = Arc::clone(&self.blockchain);
                    thread::spawn(move || {
                        Node::handle_client(stream, blockchain);
                    });
                }
                Err(e) => eprintln!("Failed to establish connection: {}", e),
            }
        }
    }

    fn handle_client(mut stream: TcpStream, blockchain: Arc<Mutex<Blockchain>>) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        let request = String::from_utf8_lossy(&buffer);
        println!("Received: {}", request);

        let response = match request.trim() {
            "GET_CHAIN" => {
                let chain = blockchain.lock().unwrap();
                serde_json::to_string(&chain.chain).unwrap()
            }
            _ => "Unknown command".to_string(),
        };

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
