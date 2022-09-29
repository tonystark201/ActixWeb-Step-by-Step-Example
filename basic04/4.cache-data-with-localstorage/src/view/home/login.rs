use std::fs;
use actix_web::HttpResponse;

fn read_file(file_path: String) -> String {
    let data: String = fs::read_to_string(file_path)
        .expect("Unable to read file");
    return data
}

pub async fn login() -> HttpResponse {

    let mut html_data = read_file(
        String::from("./templates/login.html"));
    let javascript_data: String = read_file(
        String::from("./javascript/login.js"));
    let css_data: String = read_file(
        String::from("./css/main.css"));
    let base_css_data: String = read_file(
        String::from("./css/base.css"));

    html_data = html_data.replace("{{JAVASCRIPT}}",
                                  &javascript_data);
    html_data = html_data.replace("{{CSS}}", &css_data);
    html_data = html_data.replace("{{BASE_CSS}}", &base_css_data);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}
