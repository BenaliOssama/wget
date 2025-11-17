use std::fs::File;
use std::io::{self, BufRead};
use url::Url;
use std::path::Path;
use anyhow::Result;

pub fn get_filename(url_str: &str) -> String {
    match Url::parse(url_str) {
        Ok(url) => {
            let mut filename = url.path().trim_start_matches('/').to_string();
            if filename.is_empty() {
                filename = "index.html".to_string();
            }
            if let Some(query) = url.query() {
                filename.push('?');
                filename.push_str(query);
            }
            filename
        }
        Err(_) => "invalid_url".to_string(),
    }
}


pub fn get_urls<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let urls = reader
        .lines()
        .filter_map(|line| line.ok())   // ignore lines that failed to read
        .filter(|line| !line.trim().is_empty()) // skip empty lines
        .collect::<Vec<String>>();

    Ok(urls)
}