# wget-clone (Rust)

A learning project that reimplements core features of **GNU wget** in **Rust**.
The goal is to understand networking, concurrency, filesystem handling, and website mirroring using a compiled, memory-safe language.

This is **not** a full replacement for wget.

---

## Features

* Download a file from a URL
* Save a file with a custom name
* Save a file to a specific directory
* Show download progress and statistics
* Limit download speed
* Download in background with logging
* Download multiple files asynchronously
* Mirror an entire website for offline use

---

## Supported Protocols

* HTTP
* HTTPS
  (FTP optional, depending on implementation)

---

## Requirements

* Rust (stable)
* Cargo

---

## Build & Run

Run directly with Cargo:

```sh
$ cargo run -- https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg
```

Or build a binary:

```sh
$ cargo build --release
$ ./target/release/wget-clone https://example.com/file.zip
```

---

## Usage

Basic download:

```sh
$ cargo run -- https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg
```

Example output:

```text
start at 2017-10-14 03:46:06
sending request, awaiting response... status 200 OK
content size: 56370 [~0.06MB]
saving file to: ./EMtmPFLWkAA8CIS.jpg
55.05 KiB / 55.05 KiB [==============================================] 100.00% 1.24 MiB/s 0s

Downloaded [https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg]
finished at 2017-10-14 03:46:07
```

If the response status is not `200 OK`, the program exits with an error.

---

## Flags

### Background download `-B`

Runs the download in the background and redirects output to `wget-log`.

```sh
$ cargo run -- -B https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg
Output will be written to "wget-log".
```

---

### Output filename `-O`

Save the file under a different name.

```sh
$ cargo run -- -O=meme.jpg https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg
```

---

### Output directory `-P`

Save the file to a specific directory.

```sh
$ cargo run -- -P=~/Downloads/ -O=meme.jpg https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg
```

---

### Rate limit `--rate-limit`

Limit download speed.

Supported units:

* `k` → kilobytes per second
* `M` → megabytes per second

```sh
$ cargo run -- --rate-limit=400k https://example.com/file.zip
$ cargo run -- --rate-limit=2M https://example.com/file.zip
```

---

### Multiple downloads `-i`

Download multiple files asynchronously from a text file.

```sh
$ cat download.txt
https://assets.01-edu.org/wgetDataSamples/20MB.zip
https://assets.01-edu.org/wgetDataSamples/Image_10MB.zip

$ cargo run -- -i=download.txt
```

All downloads run concurrently.

---

## Website Mirroring

Mirror an entire website locally.

```sh
$ cargo run -- --mirror https://example.com
```

Files are stored in a directory named after the domain:

```text
www.example.com/
```

---

### Mirror Flags

#### Reject file types `-R` / `--reject`

Skip downloading files with specific extensions.

```sh
$ cargo run -- --mirror -R=jpg,gif https://example.com
```

---

#### Exclude directories `-X` / `--exclude`

Skip specific paths while mirroring.

```sh
$ cargo run -- --mirror -X=/assets,/css https://example.com
```

---

#### Convert links `--convert-links`

Rewrite links so the mirrored site works offline.

```sh
$ cargo run -- --mirror --convert-links https://example.com
```

---


## Learning Goals

This project focuses on:

* Rust networking (blocking or async)
* Streaming downloads
* Async/concurrency (`tokio`, threads, or channels)
* Rate limiting
* Progress reporting
* Recursive crawling
* HTML parsing
* Safe filesystem operations
* CLI design

---

## Notes

* Error handling is explicit and strict
* Designed for Unix-like systems
* Output format intentionally mimics wget
* Features may be incomplete or evolving
