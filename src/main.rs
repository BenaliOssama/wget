use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // URL info (only HTTP, no HTTPS)
    let host = "example.com";
    let path = "/file.zip";

    // Connect to the server
    let mut stream = TcpStream::connect(format!("{}:80", host))?;

    // Send HTTP GET request
    let request = format!(
        "GET {} HTTP/1.0\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path, host
    );
    stream.write_all(request.as_bytes())?;

    // Read the response
    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;

    // Find where the header ends
    if let Some(pos) = response.windows(4).position(|w| w == b"\r\n\r\n") {
        let body = &response[pos + 4..]; // skip header
        let mut file = File::create("file.zip")?;
        file.write_all(body)?;
        println!("Downloaded file.zip successfully.");
    } else {
        println!("Failed to parse HTTP response");
    }

    Ok(())
}

