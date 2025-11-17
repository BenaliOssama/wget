use crate::parser::WgetCli;
use futures_util::future::join_all;
use std::error::Error;

use crate::utils::{get_filename, get_urls};
use anyhow::Result;
use futures_util::stream::StreamExt;
use reqwest::Client;
use std::collections::HashSet;
use std::path::Path;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use tokio::time::{Duration, sleep};
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
        let rate_limit = if let Some(limit) = self.speed_limit {
            limit
        } else {
            f64::MAX
        };

        self.log(&format!("Downloading: {}", &self.url));

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            let delay = Duration::from_secs_f64((chunk.len() as f64) / rate_limit);
            if delay > Duration::ZERO {
                sleep(delay).await;
            }
            file.write_all(&chunk).await?;
        }

        self.log(&format!("Downloaded: {}", filename));
        Ok(())
    }

    async fn download(&self) -> Result<()> {
        // Example: multiple URLs; fallback to self.url if needed
        let urls = if let Some(urls) = Some(vec![self.url.clone()]) {
            urls.clone()
        } else {
            vec![self.url.clone()]
        };

        let client = Client::new();

        let futures = urls.into_iter().map(|url| {
            let client = client.clone();
            let output_path = format!("downloaded_{}", url.split('/').last().unwrap_or("file"));
            let logger = self.clone(); // assuming self implements Clone or logging can be captured

            async move {
                logger.log(&format!("Downloading: {}", &url));

                let resp = client.get(&url).send().await.unwrap();
                let content = resp.bytes().await.unwrap();

                let mut file = File::create(&output_path).await.unwrap();
                file.write_all(&content).await.unwrap();

                logger.log(&format!("Downloaded: {}", &output_path));
            }
        });

        join_all(futures).await;

        Ok(())
    }

    async fn mirror(&self) -> anyhow::Result<()> {
        self.log(&format!("Starting mirror of: {}", &self.url));

        let base_url = Url::parse(&self.url)?;
        let mut visited_urls = HashSet::new();
        let mut urls_to_visit = vec![self.url.clone()];

        let client = Client::new();

        // Base directory = hostname or "mirror"
        let base_url_dir = base_url.host_str().unwrap_or("mirror");
        fs::create_dir_all(base_url_dir).await?;

        while let Some(current_url) = urls_to_visit.pop() {
            if visited_urls.contains(&current_url) {
                continue;
            }
            visited_urls.insert(current_url.clone());
            self.log(&format!("Mirroring: {}", current_url));

            let response = match client.get(&current_url).send().await {
                Ok(r) => r,
                Err(e) => {
                    self.log(&format!("Failed to request {}: {}", current_url, e));
                    continue;
                }
            };

            if !response.status().is_success() {
                self.log(&format!(
                    "Failed to download: {} (Status: {})",
                    current_url,
                    response.status()
                ));
                continue;
            }

            // Setup download stream
            let mut stream = response.bytes_stream();
            let rate_limit = self.speed_limit.unwrap_or(f64::MAX);

            // Parse URL to build local path
            let parsed_url = Url::parse(&current_url)?;
            let path = parsed_url.path();
            let local_path = if path == "/" || path.is_empty() {
                "index.html".to_string()
            } else {
                let mut p = path
                    .trim_start_matches('/')
                    .replace("?", "_")
                    .replace("#", "_");
                if p.ends_with('/') {
                    p.push_str("index.html");
                } else if Path::new(&p).extension().is_none() {
                    p.push_str(".html");
                }

                p
            };

            // Final filename = base_dir + local_path
            let filename = format!("{}/{}", base_url_dir, local_path);
            if let Some(parent) = Path::new(&filename).parent() {
                fs::create_dir_all(parent).await?;
            }

            self.log(&format!("Saving: {}", filename));
            let mut file = File::create(&filename).await?;
            let mut content = Vec::new();

            // Download with optional rate limiting
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                if rate_limit.is_finite() && rate_limit > 0.0 {
                    let delay = Duration::from_secs_f64((chunk.len() as f64) / rate_limit);
                    if delay > Duration::ZERO {
                        sleep(delay).await;
                    }
                }
                file.write_all(&chunk).await?;
                content.extend_from_slice(&chunk);
            }

            // Parse links only if looks like HTML
            let content_str = String::from_utf8_lossy(&content);
            let lowered = content_str.to_ascii_lowercase();
            if lowered.contains("<html") || lowered.contains("<!doctype") {
                let links = extract_links(&content_str, &base_url);
                for link in links {
                    if !visited_urls.contains(&link) {
                        if let Ok(u) = Url::parse(&link) {
                            if u.host() == base_url.host() {
                                urls_to_visit.push(link);
                            }
                        }
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
                eprintln!("{}", message); // stderr for background
            } else {
                println!("{}", message); // stdout for foreground
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
