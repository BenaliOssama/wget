mod executer;
mod parser;
mod utils;
use tokio;
use parser::*;
use executer::*;
use std::thread;

#[tokio::main]

//cargo run  -- --limit-rate=200<k can be use here to 
// show the limit..> https://example.com/file.zip 

async fn main() {

    println!("Download started in background"); 
    thread::spawn(|| {
        // call your async runtime here if needed
        tokio::runtime::Runtime::new().unwrap().block_on(async {

    /*______________________________________________________*/


    let matches = parse_args();
    let mut wget_cli = WgetCli::new(&matches);
    wget_cli.handle_destination();
    println!("{:?}", wget_cli);
    if let Err(err) = wget_cli.execute().await {
        println!("in the main : {}", err);
    }
    /*______________________________________________________*/


        });
    });
    println!("Download is going in background"); 
}
