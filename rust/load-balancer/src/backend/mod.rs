pub mod http;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliArgs {
    #[arg(short, long, value_delimiter = ',', num_args = 1..)]
    pub ports: Vec<u16>,

    // #[arg(short, long, default_value = "8080")]
    // pub bind: u16,
}



