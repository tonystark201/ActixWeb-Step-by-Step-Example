use actix_web::http::StatusCode;
use actix_web::HttpResponse;

const LOGIN: &'static str = r#"
<!doctype html>
<html>
    <head>
        <title>Actix Web</title>
    </head>
    <body>
        <h1>V1:Login</h>
        <p>Hello, welcome to actix web<p>
    </body>
</html>
"#;

pub async fn login() -> HttpResponse {
    let res = HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(LOGIN);
    res
}
