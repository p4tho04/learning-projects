use std::net::{ TcpListener, TcpStream };
use std::io::{ BufReader, BufRead, Write };
use std::sync::{ Arc, Mutex };
use std::thread;
use serde::{ Deserialize, Serialize };

// enum HttpMethod {
//     GET,
//     POST,
//     PUT,
//     DELETE,
// }

// trait Http {
//     fn handle_http_request(stream: TcpStream) -> Result<HttpRequest>;
// }

#[derive(Debug)]
struct HttpRequest {
    method: String,
    path: String,
    version: String,
}

#[derive(Serialize, Deserialize)]
struct HttpResponse {
    status: String,
    message: String,
}

pub struct HttpServer {
    port: u16,
    listener: TcpListener,
    connection_count: Arc<Mutex<u32>>,
}

impl HttpServer {
    pub fn new(port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&addr)?;

        Ok(HttpServer {
            port,
            listener,
            connection_count: Arc::new(Mutex::new(0)),
        })
    }

    fn increase_connection_count(count_lock: &Arc<Mutex<u32>>, port: &u16) {
        let mut count = count_lock.lock().unwrap();
        *count += 1;
        println!("CONNECT (127.0.0.1:{}) - Connection count: {}", port, *count);
    }

    fn decrease_connection_count(count_lock: &Arc<Mutex<u32>>, port: &u16) {
        let mut count = count_lock.lock().unwrap();
        *count -= 1;
        println!("DISCONNECT (127.0.0.1:{}) - Connection count: {}", port, *count);
    }

    fn handle_http_request(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        let local_addr = stream.local_addr()?;
        let mut reader = BufReader::new(&stream);
        let mut user_http_request = String::new();
        reader.read_line(&mut user_http_request)?;

        // Write JSON to the stream
        let response_message = format!("Hello from {}", local_addr);
        let response = HttpResponse {
            status: "Ok".to_string(),
            message: response_message,
        };
        let json = serde_json::to_string(&response)?;
        stream.write_all(json.as_bytes())?;
        stream.flush()?;

        let parts: Vec<&str> = user_http_request.split_whitespace().collect();
        if parts.len() != 3 {
            return Err("Invalid HTTP request".into());
        }

        let http_request = HttpRequest {
            method: parts[0].to_string(),
            path: parts[1].to_string(),
            version: parts[2].to_string(),
        };

        println!("\nREQUEST ({})\n\tmethod: {}\n\tpath: {}\n\tversion: {}\n", local_addr, http_request.method, http_request.path, http_request.version);

        Ok(())
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("BOOTING - Listening on {}...", self.listener.local_addr()?);

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let connection_count_lock = Arc::clone(&self.connection_count);
                    let port = self.port;

                    // Handle multiple connections to same port
                    thread::spawn(move || {
                        Self::increase_connection_count(&connection_count_lock, &port); // Track incoming connections

                        if let Err(e) = Self::handle_http_request(stream) {
                            eprintln!("ERROR (127.0.0.1:{}) - Error handling client: {}", port, e);
                        }

                        Self::decrease_connection_count(&connection_count_lock, &port); // Track leaving connections
                    });
                },

                Err(e) => {
                    let port = self.port;
                    eprintln!("ERROR (127.0.0.1:{}) - Connection failed: {}", port, e);
                },
            }
        }

        Ok(())
    }
}

