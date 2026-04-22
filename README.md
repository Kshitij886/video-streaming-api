# Actix Web Video Streaming (HTTP Range Requests)

A minimal Actix Web server that streams an `.mp4` file using HTTP `Range` requests. This enables seeking/scrubbing in most browsers and video players by returning `206 Partial Content` with a proper `Content-Range` header.

## Features

- Streams `videos/sample.mp4`
- Supports `Range: bytes=start-end` requests
- Returns:
  - `206 Partial Content` for range responses
  - `Content-Range` header
  - `Content-Type: video/mp4`

## Project Structure

```text
.
├── src
│   └── main.rs
└── videos
    └── sample.mp4

Requirements

    Rust (stable) + Cargo

Install Rust: https://rustup.rs/
Setup

    Create a new Rust project (if you haven’t already):

Bash

cargo new actix-video-stream
cd actix-video-stream

    Add dependencies to Cargo.toml:

toml

[dependencies]
actix-web = "4"

    Place your video file at:

text

videos/sample.mp4

    The server expects this path relative to the working directory where you run the binary.

Run

Bash

cargo run

Server will start at:

text

http://127.0.0.1:8080

Usage
Stream endpoint

    GET /stream

Example:

Bash

curl -v http://127.0.0.1:8080/stream --output out.mp4

Range request example (partial download)

Bash

curl -v \
  -H "Range: bytes=0-999999" \
  http://127.0.0.1:8080/stream \
  --output part.mp4

You should see a response similar to:

    Status: 206 Partial Content
    Header: Content-Range: bytes 0-999999/<total_size>

Play in a browser

Open:

text

http://127.0.0.1:8080/stream

Or embed in HTML:

HTML

<video controls width="800" src="http://127.0.0.1:8080/stream"></video>

Notes / Limitations

    This implementation reads the requested range into memory (Vec<u8>) before responding. For large ranges or high concurrency, you’ll want a true streaming body instead of buffering the full chunk.
    The code always responds with PartialContent even when there is no Range header. Many clients still work fine, but you may want to return 200 OK when serving the full file.
    Minimal validation is performed on the Range header (e.g., out-of-bounds ranges are not fully handled).

License

MIT (or choose your preferred license)
