use std::fs::File;
use std::io::copy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://example.com/file.zip";

    // async request
    let response = reqwest::get(url).await?;

    // get response body as bytes
    let bytes = response.bytes().await?;

    // write to file
    let mut file = File::create("file.zip")?;
    copy(&mut bytes.as_ref(), &mut file)?;

    println!("Downloaded file.zip successfully.");
    Ok(())
}

