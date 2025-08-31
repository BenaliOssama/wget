use crate::parser::WgetCli;
use crate::utils::{get_filename, get_urls};
use futures_util::stream::StreamExt;
use reqwest::Client;
use anyhow::Result;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::time::{Duration, sleep};
use std::collections::HashSet;
use std::path::Path;
use url::Url;

pub trait Executer {
    async fn execute(&mut self) -> Result<()>;
    async fn download(&self) -> Result<()>;
    async fn apply_speed_limit(&self) -> Result<()>;
    async fn mirror(&self) -> Result<()>;
    fn log(&self, message: &str);
}

impl Executer for WgetCli {
    async fn execute(&mut self) -> Result<()> {
        if self.mirror {
            self.log("heey");
        }
        println!("hello");
        // Handle background mode
        if self.background {
            self.log("Continuing in background...");
        }

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

        let mut file = File::create(&self.output).await?;
        let mut stream = response.bytes_stream();
        let rate_limit = if let Some(limit) = self.speed_limit { limit } else { f64::MAX };

        self.log(&format!("Downloading: {}", &self.url));

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            let delay = Duration::from_secs_f64(chunk.len() as f64 / rate_limit);
            if delay > Duration::ZERO {
                sleep(delay).await;
            }
            file.write_all(&chunk).await?;
        }

        self.log(&format!("Downloaded: {}", filename));
        Ok(())
    }

    async fn download(&self) -> Result<()> {
        self.log(&format!("Downloading: {}", &self.url));
        
        let resp = reqwest::get(&self.url).await?;
        let content = resp.text().await?;

        let mut file = File::create(&self.output).await?;
        file.write_all(content.as_bytes()).await?;

        self.log(&format!("Downloaded: {}", &self.output));
        Ok(())
    }

    async fn mirror(&self) -> Result<()> {
        self.log(&format!("Starting mirror of: {}", &self.url));
        
        let base_url = Url::parse(&self.url)?;
        let mut visited_urls = HashSet::new();
        let mut urls_to_visit = vec![self.url.clone()];
        
        let client = Client::new();

        while let Some(current_url) = urls_to_visit.pop() {
            if visited_urls.contains(&current_url) {
                continue;
            }
            
            visited_urls.insert(current_url.clone());
            
            self.log(&format!("Mirroring: {}", current_url));
            
            let response = client.get(&current_url).send().await?;
            
            if !response.status().is_success() {
                self.log(&format!("Failed to download: {} (Status: {})", current_url, response.status()));
                continue;
            }
            
            // Apply speed limit to mirror downloads
            let mut stream = response.bytes_stream();
            let rate_limit = if let Some(limit) = self.speed_limit { limit } else { f64::MAX };
            
            // Create directory structure based on URL path
            let parsed_url = Url::parse(&current_url)?;
            let path = parsed_url.path();
            let local_path = format!(".{}", path);
            
            if let Some(parent) = Path::new(&local_path).parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
            
            let filename = if path.ends_with('/') || path.is_empty() {
                format!("{}index.html", local_path)
            } else {
                local_path
            };
            
            let mut file = File::create(&filename).await?;
            let mut content = Vec::new();
            
            // Download with speed limit
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                let delay = Duration::from_secs_f64(chunk.len() as f64 / rate_limit);
                if delay > Duration::ZERO {
                    sleep(delay).await;
                }
                file.write_all(&chunk).await?;
                content.extend_from_slice(&chunk);
            }
            
            // Extract links from HTML content for further mirroring
            let content_str = String::from_utf8_lossy(&content);
            if content_str.contains("<html") || content_str.contains("<!DOCTYPE") {
                let links = extract_links(&content_str, &base_url);
                for link in links {
                    if !visited_urls.contains(&link) && 
                       Url::parse(&link).map(|u| u.host() == base_url.host()).unwrap_or(false) {
                        urls_to_visit.push(link);
                    }
                }
            }
            
            self.log(&format!("Saved: {}", filename));
        }
        
        self.log("Mirror completed");
        Ok(())
    }

    fn log(&self, message: &str) {
        if !self.quiet {
            if self.background {
                eprintln!("{}", message);  // stderr for background
            } else {
                println!("{}", message);   // stdout for foreground
            }
        }
    }
}

// Helper function to extract links from HTML content
fn extract_links(html: &str, base_url: &Url) -> Vec<String> {
    let mut links = Vec::new();
    
    // Simple regex-based link extraction (in a real implementation, use an HTML parser)
    let link_patterns = [
        r#"href\s*=\s*["']([^"']+)["']"#,
        r#"src\s*=\s*["']([^"']+)["']"#,
    ];
    
    for pattern in &link_patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            for cap in re.captures_iter(html) {
                if let Some(link) = cap.get(1) {
                    let link_str = link.as_str();
                    
                    // Convert relative URLs to absolute
                    if let Ok(absolute_url) = base_url.join(link_str) {
                        links.push(absolute_url.to_string());
                    }
                }
            }
        }
    }
    
    links
}