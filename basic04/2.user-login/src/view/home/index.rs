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
    let css: String = read_file(
        String::from("./css/main.css")
    );
    let base_css: String = read_file(
        String::from("./css/base.css")
    );

    html = html.replace("{{JAVASCRIPT}}", &js);
    html = html.replace("{{CSS}}", &css);
    html = html.replace("{{BASE_CSS}}", &base_css);

    println!("{:?}",&html);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn index()->Result<impl Responder> {
    Ok(NamedFile::open("templates/index.html")?)
}
