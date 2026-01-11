use clap::Parser;
use load_balancer::backend::{ CliArgs, http::HttpServer };
use std::io::Result;
use std::thread;

fn main() -> Result<()> {
    let args = CliArgs::parse();

    let mut handles = Vec::new();

    for &port in &args.ports {
        let server = HttpServer::new(port)?;

        let handle = thread::spawn(move || {
            if let Err(e) = server.run() {
                eprintln!("Server on port {} failed: {}", port, e);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
