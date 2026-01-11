use std::net::{ TcpListener, TcpStream };
use std::io::{ BufReader, BufRead, Result };
use std::sync::{ Arc, Mutex };
use std::thread;

// enum HttpMethod {
//     GET,
//     POST,
//     PUT,
//     DELETE,
// }

#[derive(Debug)]
pub struct HttpRequest {
    method: String,
    path: String,
    version: String,
}

// pub struct HttpResponse {
//     body: Option<String>,
// }

pub struct HttpServer {
    port: u16,
    listener: TcpListener,
    connection_count: Arc<Mutex<u32>>,
}

fn handle_http_request(stream: TcpStream) -> Result<HttpRequest> {
    // let local_addr = stream.local_addr()?;
    // let mut reader = BufReader::new(stream);

    // let mut line = String::new();
    // let mut user_http_request = String::new();

    // while let Ok(len) = reader.read_line(&mut line) {
    //     if len == 0 {
    //         break;
    //     }

    //     println!("RECEIVED - {} received: {}", local_addr, line.trim());
    //     user_http_request.push_str(&line);
    //     line.clear();
    // }


    let local_addr = stream.local_addr()?;
    let mut reader = BufReader::new(stream);
    let mut user_http_request = String::new();
    reader.read_line(&mut user_http_request)?;

    println!("RECEIVED - {} from {}", user_http_request, local_addr);
    let parts: Vec<&str> = user_http_request.split_whitespace().collect();

    if parts.len() != 3 {
        panic!("ERROR - Invalid HTTP request line.");
    }

    let http_request = HttpRequest {
        method: parts[0].to_string(),
        path: parts[1].to_string(),
        version: parts[2].to_string(),
    };

    println!("REQUEST - {:?}\n\tmethod: {}\n\tpath: {}\n\tversion: {}", http_request, http_request.method, http_request.path, http_request.version);

    Ok(http_request)
}

fn increase_connection_count(count_lock: &Arc<Mutex<u32>>, port: &u16) {
    let mut count = count_lock.lock().unwrap();
    *count += 1;
    println!("CONNECT - Port {} connection count: {}", port, *count);
}

fn decrease_connection_count(count_lock: &Arc<Mutex<u32>>, port: &u16) {
    let mut count = count_lock.lock().unwrap();
    *count -= 1;
    println!("DISCONNECT - Port {} connection count: {}", port, *count);
}

impl HttpServer {
    pub fn new(port: u16) -> Result<Self> {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&addr)?;

        Ok(HttpServer {
            port,
            listener,
            connection_count: Arc::new(Mutex::new(0)),
        })
    }

    pub fn run(&self) -> Result<()> {
        println!("BOOTING - Listening on {}...", self.listener.local_addr()?);

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let connection_count_lock = Arc::clone(&self.connection_count);
                    let port = self.port;

                    // Handle multiple connections to same port
                    thread::spawn(move || {
                        increase_connection_count(&connection_count_lock, &port); // Track incoming connections

                        if let Err(e) = handle_http_request(stream) {
                            eprintln!("ERROR - Error handling client on port {}: {}", port, e);
                        }

                        decrease_connection_count(&connection_count_lock, &port); // Track leaving connections
                    });
                },

                Err(e) => {
                    eprintln!("ERROR - Connection failed: {}", e);
                },
            }
        }

        Ok(())
    }
}

