use crate::parser::WgetCli;
use crate::utils::{get_filename, get_urls};
use futures_util::stream::StreamExt;
use reqwest::Client;
use anyhow::Result;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::time::{Duration, sleep};


pub trait Executer {
    async fn execute(&mut self) -> Result<()>;
    async fn download(&self)-> Result<()>;
    async fn apply_speed_limit(&self) -> Result<()>;
    async fn mirror(&self) -> Result<()>;
}

impl Executer for WgetCli {
    async fn execute(&mut self) -> Result<()> {
        if let Some(urls_file) = self.urls_file.clone() {
            let urls = get_urls(urls_file)?;
            for url in urls {
                self.output = get_filename(&url);
                self.url = url;
                self.download().await?;
            }
            return Ok(());
        }
        if self.mirror {
            self.mirror().await?;
            return Ok(());
        }
        self.apply_speed_limit().await?;
        Ok(())
    }

    async fn apply_speed_limit(&self) -> Result<()> {
        let client = Client::new();
        let response = client.get(&self.url).send().await?;
        let filename = self.dest.clone().unwrap_or_else(|| get_filename(&self.url));

        let mut file = File::create(&self.output).await.unwrap();

        let mut stream = response.bytes_stream();

        let rate_limit = if let Some(limit) = self.speed_limit {limit} else {f64::MAX};

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

    async fn download(&self) -> Result<()>{
        let resp = reqwest::get(&self.url).await?;
        let content = resp.text().await?;

        let mut file = File::create(&self.output).await.unwrap();
        file.write_all(content.as_bytes()).await.unwrap();

        Ok(())
    }

    async fn mirror(&self) -> Result<()> {
        Ok(())
    }
}
