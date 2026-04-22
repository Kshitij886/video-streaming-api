#![allow(unused)]
use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
};

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, Result, web};

async fn stream_video(req: HttpRequest) -> Result<impl Responder> {
    let file_path = "videos/sample.mp4";
    let file = File::open(file_path)?;
    let metadata = file.metadata()?;
    let total_size = metadata.len();

    let range_header = req
        .headers()
        .get("Range")
        .map(|h| h.to_str().ok())
        .flatten();
    let (start, end) = if let Some(range) = range_header {
        let parts: Vec<&str> = range.trim_start_matches("bytes=").split('-').collect();
        let start = parts[0].parse::<u64>().unwrap_or(0);
        let end = parts
            .get(1)
            .and_then(|&s| s.parse::<u64>().ok())
            .unwrap_or(total_size - 1);
        (start, end)
    } else {
        (0, total_size - 1)
    };
    let mut reader = BufReader::new(file);
    reader.seek(SeekFrom::Start(start))?;
    let mut buffer = Vec::new();
    let length = (end - start + 1) as usize;
    reader.take(length as u64).read_to_end(&mut buffer)?;
    Ok(HttpResponse::PartialContent()
        .content_type("video/mp4")
        .append_header((
            "Content-Range",
            format!("bytes {}-{}/{}", start, end, total_size),
        ))
        .body(buffer))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/stream", web::get().to(stream_video)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
