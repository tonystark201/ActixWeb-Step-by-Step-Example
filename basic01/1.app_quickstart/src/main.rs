use actix_web::{App, HttpResponse, HttpServer, middleware, web};
use actix_web::http::StatusCode;
use log::{info};

const LOGIN: &'static str = r#"
<!doctype html>
<html>
    <head>
        <title>Actix Web</title>
    </head>
    <body>
        <h1>Login</h>
        <p>Hello, welcome to actix web<p>
    </body>
</html>
"#;

const LOGOUT: &'static str = r#"
<!doctype html>
<html>
    <head>
        <title>Actix Web</title>
    </head>
    <body>
        <h1>Logout</h>
        <p>Good bye!<p>
    </body>
</html>
"#;

pub async fn login() -> HttpResponse {
    let res = HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(LOGIN);
    res
}

pub async fn logout() -> HttpResponse {
    let res = HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(LOGOUT);
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
            .service(
                web::scope("v1")
                    .service(
                    web::resource("/login")
                        .route(web::get().to(login))
                ).service(
                    web::resource("/logout")
                        .route(web::get().to(logout))
                )
            );
        return app
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
