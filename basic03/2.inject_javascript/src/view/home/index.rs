use std::fs;
use actix_files::NamedFile;
use actix_web::{HttpResponse, Responder, Result};


fn read_file(file_path: String) -> String {
    let data: String = fs::read_to_string(file_path)
        .expect("Unable to read file");
    return data
}

pub async fn home() -> HttpResponse {
    let mut html = read_file(
        String::from("./templates/index.html")
    );
    let js = read_file(
        String::from("./javascript/main.js")
    );
    html = html.replace("{{JAVASCRIPT}}", &js);

    println!("{:?}",&html);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn index()->Result<impl Responder> {
    Ok(NamedFile::open("templates/index.html")?)
}
