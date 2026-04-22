use std::{
    fs::File,
    io::{BufReader, Read},
};

use actix_web::{App, HttpResponse, HttpServer, Responder, Result, web};

async fn stream_video() -> Result<impl Responder> {
    let file_path = "videos/sample.mp4";
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(HttpResponse::Ok().content_type("video/mp4").body(buffer))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/stream", web::get().to(stream_video)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
