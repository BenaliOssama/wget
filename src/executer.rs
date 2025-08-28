use crate::parser::WgetCli;
use reqwest;
use std::fs::File;
use std::io::{self, Write};
pub trait Executer {
    async fn execute(&self) -> Result<(), reqwest::Error>;
    fn download(&self);
    fn apply_speed_limit(&self);
    fn mirror(&self);
}

impl Executer for WgetCli {
    async fn execute(&self) -> Result<(), reqwest::Error> {
        let resp = reqwest::get(&self.url).await?;
        let content = resp.text().await?;
        // instead of unwrap we should handle the error properly
        let mut file = File::create(&self.output).unwrap();
        write!(file, "{}", content).unwrap();        
        Ok(())
    }

    fn apply_speed_limit(&self) {}

    fn download(&self) {}

    fn mirror(&self) {}
}
