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

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn index()->Result<impl Responder> {
    Ok(NamedFile::open("templates/index.html")?)
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use crate::view::home::index::home;

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(
            App::new().route("/", web::get().to(home))
        ).await;
        let req = test::TestRequest::default().to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
