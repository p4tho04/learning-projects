use super::http::{ HttpServer };
use std::net::{ TcpListener, TcpStream };
use std::thread;
use std::sync::{ Arc, atomic::AtomicU16, atomic::Ordering };
use std::io::{  BufReader, BufRead, Write, Read };

pub struct LoadBalancer {
    port: u16,
    listener: TcpListener,
    server_ports: Vec<u16>,
    current_idx: Arc<AtomicU16>,
}

impl LoadBalancer {
    pub fn new(bind_port: u16, server_ports: Vec<u16>) -> Result<Self, Box<dyn std::error::Error>> {
        let addr = format!("127.0.0.1:{}", bind_port);
        let listener = TcpListener::bind(&addr)?;

        Ok(LoadBalancer { port: bind_port, listener, server_ports, current_idx: Arc::new(AtomicU16::new(0)) })
    }

    pub fn handle_http_request(mut stream: TcpStream, current_idx: Arc<AtomicU16>, server_port: u16) -> Result<(), Box<dyn std::error::Error>> {
        current_idx.fetch_add(1, Ordering::Relaxed);
        let server_addr = format!("127.0.0.1:{}", server_port);
        let mut user_reader = BufReader::new(&stream);
        let mut user_http_request = String::new();
        user_reader.read_line(&mut user_http_request)?;
        println!("LB Recevied: {}", user_http_request);

        let mut server_stream = TcpStream::connect(server_addr)?;
        server_stream.write_all(user_http_request.as_bytes());
        let mut server_reader = BufReader::new(&server_stream);
        let mut server_response = String::new();
        server_reader.read_line(&mut server_response)?;

        stream.write_all(server_response.as_bytes())?;
        server_stream.flush()?;


        Ok(())
    }

    pub fn run_loadbalancer(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("BOOTING Load Balancer - Listening on {}...", self.listener.local_addr()?);

        for stream in self.listener.incoming() {
            let idx = Arc::clone(&self.current_idx);
            if self.current_idx.load(Ordering::Relaxed) as usize == self.server_ports.len() {
                self.current_idx.fetch_and(0, Ordering::Relaxed);
            }
            let server_port = self.server_ports.get(self.current_idx.load(Ordering::Relaxed) as usize).ok_or("can't find server address")?.clone();

            match stream {
                Ok(stream) => {
                    let port = self.port;

                    // Handle multiple connections to same port
                    thread::spawn(move || {
                        if let Err(e) = Self::handle_http_request(stream, idx, server_port) {
                            eprintln!("ERROR (127.0.0.1:{}, LB) - Error handling client: {}", port, e);
                        }
                    });
                },

                Err(e) => {
                    let port = self.port;
                    eprintln!("ERROR (127.0.0.1:{}, LB) - Connection failed: {}", port, e);
                },
            }
        }

        Ok(())
    }

    pub fn run_servers(&self) -> Result<Vec<std::thread::JoinHandle<()>>, Box<dyn std::error::Error>>   {
        let mut handles = Vec::new();

        for &port in &self.server_ports {
            let server = HttpServer::new(port)?;

            let handle = thread::spawn(move || {
                if let Err(e) = &server.run() {
                    eprintln!("Server on port {} failed: {}", port, e);
                }
            });

            handles.push(handle);
        }

        Ok(handles)
    }
}