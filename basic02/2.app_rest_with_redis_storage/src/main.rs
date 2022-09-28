mod view;
mod serialization;
mod cache;
mod app_data;

use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use actix_web::{App, HttpResponse, HttpServer, middleware, web};
use actix_web::http::StatusCode;
use log::{info, trace};
use r2d2_redis::RedisConnectionManager;
use app_data::AppData;

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

    /*
        http://127.0.0.1:8080/v1/login
        http://127.0.0.1:8081/v2/login
    */
    env_logger::init();
    info!("Actix Microservice - v0.1.0");
    trace!("Starting...");


    dotenv().ok();
    let redis_addr= env::var("REDIS")
        .unwrap_or_else(|_| "redis://localhost:16379/1".into())
        .parse::<String>()
        .unwrap();

    let manager = RedisConnectionManager::new(redis_addr).unwrap();
    let pool = r2d2::Pool::new(manager).unwrap();

    let app_data = web::Data::new(
        AppData{
            pool: Arc::new(pool)
        }
    );

    info!("starting HTTP server at http://localhost:8080");
    HttpServer::new(move || {
        let app = App::new()
            .app_data(app_data.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .configure(view::route_config)
            .default_service(web::route().to(not_found));
        return app
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
