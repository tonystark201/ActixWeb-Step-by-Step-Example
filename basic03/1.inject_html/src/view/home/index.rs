use std::fs;
use actix_files::NamedFile;
use actix_web::{HttpResponse, Responder, Result};


fn read_file(file_path: String) -> String {
    let data: String = fs::read_to_string(file_path)
        .expect("Unable to read file");
    return data
}

pub async fn home() -> HttpResponse {
    let html_data = read_file(
        String::from("./templates/index.html")
    );
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}

pub async fn index()->Result<impl Responder> {
    Ok(NamedFile::open("templates/index.html")?)
}
