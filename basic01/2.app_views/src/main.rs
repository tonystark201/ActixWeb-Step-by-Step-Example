mod view;

use actix_web::{App, HttpResponse, HttpServer, middleware, web};
use actix_web::http::StatusCode;
use log::{info};

const NOTFOUND: &'static str = r#"
<!doctype html>
<html>
    <head>
        <title>Actix Web</title>
    </head>
    <body>
        <h1>404</h>
        <p>Not Found<p>
    </body>
</html>
"#;

pub async fn not_found() -> HttpResponse {
    let res = HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("text/html; charset=utf-8")
        .body(NOTFOUND);
    res
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        let app = App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .configure(view::views_factory)
            .default_service(web::route().to(not_found));
        return app
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
