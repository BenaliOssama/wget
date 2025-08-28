use reqwest::blocking::get;
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

pub fn save(content: String, output: String) -> Result<()> {
    let mut file = File::create(output)?;
    write!(file, "{}", content)?;
    Ok(())
}

pub fn download(url: &str) -> Result<String> {
    let response = get(url).unwrap();
    let body = response.text().unwrap();
    Ok(body)
}

pub fn background_downloading() {
    let url = "https://www.youtube.com/watch?v=Wx5cvXEwcy0";
    let output = get_filename(url);
    let response = download(url);
    if response.is_ok() {
        save(response.unwrap(), output).expect("error while saving the file");
    }
}
