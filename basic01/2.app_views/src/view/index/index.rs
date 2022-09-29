use actix_web::http::StatusCode;
use actix_web::HttpResponse;

const INDEX: &'static str = r#"
<!doctype html>
<html>
    <head>
        <title>Actix Web</title>
    </head>
    <body>
        <h1>Index</h>
        <p>Hello,This is Home page<p>
    </body>
</html>
"#;

pub async fn index() -> HttpResponse {
    let res = HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(INDEX);
    res
}
