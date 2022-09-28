use actix_web::http::StatusCode;
use actix_web::HttpResponse;

const LOGOUT: &'static str = r#"
<!doctype html>
<html>
    <head>
        <title>Actix Web</title>
    </head>
    <body>
        <h1>V2:Logout</h>
        <p>Good bye!<p>
    </body>
</html>
"#;

pub async fn logout() -> HttpResponse {
    let res = HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(LOGOUT);
    res
}