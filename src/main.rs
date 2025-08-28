mod executer;
mod parser;
use tokio;
use parser::*;
use executer::*;

#[tokio::main]

async fn main() {
    let matches = parse_args();
    let mut wget_cli = WgetCli::new(&matches);
    wget_cli.handle_destination();
    if let Err(err) = wget_cli.execute().await {
        println!("in the main function: {}", err);
    }
}
