use std::fs::OpenOptions;
use daemonize::Daemonize;
mod file;
use file::background_downloading;

fn main() {
    let stdout = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open("wget-log")
        .unwrap();
    let bg_process = Daemonize::new().stdout(stdout).working_directory(".");
    match bg_process.start() {
	Ok(_) => {
	    background_downloading();
	}
	Err(_) => {
	    println!("error");
	}
    }
}

fn download() {
	
}
