use std::fs::File;
use std::io::{Result, Write};
use url::Url;

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


