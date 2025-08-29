mod executer;
mod parser;
mod utils;
use tokio;
use parser::*;
use executer::*;

#[tokio::main]

//cargo run  -- --limit-rate=200<k can be use here to 
// show the limit..> https://example.com/file.zip 

async fn main() {
    let matches = parse_args();
    let mut wget_cli = WgetCli::new(&matches);
    wget_cli.handle_destination();
    if let Err(err) = wget_cli.execute().await {
        println!("in the main function: {}", err);
    }
    if let Err(err) = wget_cli.apply_speed_limit().await {
        println!("in the main function: {}", err);
    }
}
