use clap::Parser;
use load_balancer::backend::{ CliArgs, loadbalancer::LoadBalancer };
use std::thread;

fn main() -> Result<(), Box <dyn std::error::Error>> {
    let args = CliArgs::parse();

    let lb = LoadBalancer::new(args.bind_port, args.ports)?;
    let mut handles = Vec::new();
    let server_handles = lb.run_servers()?;
    let lb_handle = thread::spawn(move || {
        if let Err(e) = &lb.run_loadbalancer() {
            eprintln!("Load balancer failed: {}", e);
        }
    });

    handles.push(lb_handle);
    handles.extend(server_handles);

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
