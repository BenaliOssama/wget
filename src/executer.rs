use crate::parser::WgetCli;
use crate::utils::get_filename;

use std::error::Error;


use reqwest::Client;
use futures_util::stream::StreamExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::time::{sleep, Duration};


// pub struct WgetCli {
//     pub url: String,
//     pub output: String,
//     pub dest: Option<String>,
//     pub speed_limit: Option<String>, // f64;
//     pub background: bool,
//     pub quiet: bool,
//     pub mirror: bool,
// }


pub trait Executer {
    async fn execute(&self) -> Result<(), reqwest::Error>;
    fn download(&self);
    async fn apply_speed_limit(&self)-> Result<(), Box<dyn Error>>;
    fn mirror(&self);
}

impl Executer for WgetCli {


async fn execute(&self) -> Result<(), reqwest::Error> {
    let resp = reqwest::get(&self.url).await?;
    let content = resp.text().await?;

    let mut file = File::create(&self.output).await.unwrap();
    file.write_all(content.as_bytes()).await.unwrap();

    Ok(())
}


    async fn apply_speed_limit(&self) -> Result<(), Box<dyn Error>> {
        let client = Client::new();
        let response = client.get(&self.url).send().await?;
        let filename = self.dest.clone().unwrap_or_else(|| get_filename(&self.url));

        let mut file = File::create(&self.output).await.unwrap();

        let mut stream = response.bytes_stream();

        let rate_limit = if let Some(ref limit) = self.speed_limit {
            limit.parse::<usize>()?
        } else {
            usize::MAX
        };

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            let delay = Duration::from_secs_f64(chunk.len() as f64 / rate_limit as f64);
            if delay > Duration::ZERO {
                sleep(delay).await;
            }
            file.write_all(&chunk).await?;
        }

        println!("Downloaded {}", filename);
        Ok(())
    }


    fn download(&self) {}

    fn mirror(&self) {}
}
