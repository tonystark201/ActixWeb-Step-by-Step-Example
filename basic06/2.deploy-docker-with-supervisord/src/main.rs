#[macro_use]
extern crate diesel;

mod view;
mod serialization;
mod dao;
mod app_data;
pub mod models;
pub mod schema;
mod auth;
mod errors;

use dotenv::dotenv;
use std::env;
use std::fmt::format;
use std::sync::Arc;
use actix_web::{App, http, HttpResponse, HttpServer, middleware, web};
use actix_web::http::StatusCode;
use actix_service::Service;
use actix_web::dev::ServiceResponse;
use diesel::{Connection, PgConnection};
use log::{info, trace};
use r2d2_redis::RedisConnectionManager;
use uuid::Uuid;
use app_data::AppData;
use clap::Parser;

use futures::{future,FutureExt};


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

#[derive(Parser)]
#[clap(name = "ActixWebApp")]
#[clap(author = "xxx")]
#[clap(version = "1.0")]
#[clap(about = "For ArgParse", long_about = None)]
struct Cli {
    #[
    clap(
    long,
    short='p',
    value_parser,
    default_value_t = String::from("8080")
    )
    ]
    port: String
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    /*
        Setup the Miagraion:
        1.Run: cargo install diesel_cli --no-default-features --features postgres
          Note: You must install libpq before install diesel for postgresql
        2.Run: diesel setup --database-url postgres://postgres:postgres@localhost:15432/postgres
        3.Run:
            diesel migration generate create_teachers
            diesel migration generate create_students
            diesel migration generate create_users
        4.Edit the up.sql and down.sql
        5.Run: diesel migration run --database-url postgres://postgres:postgres@localhost:15432/postgres

    */
    env_logger::init();
    info!("Actix Microservice - v0.1.0");
    trace!("Starting...");

    dotenv().ok();
    let redis_addr= env::var("REDIS")
        .unwrap_or_else(|_| "redis://localhost:16379/1".into())
        .parse::<String>()
        .unwrap();

    let db_addr= env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:15432/postgres".into())
        .parse::<String>()
        .unwrap();

    let cli:Cli = Cli::parse();
    println!("{:?}",cli.port);

    let address:String = format!("0.0.0.0:{}",cli.port.as_str());

    info!("starting HTTP server at {}",address.as_str());

    HttpServer::new(move || {
        let manager = RedisConnectionManager::new(redis_addr.clone()).unwrap();
        let pool = r2d2::Pool::new(manager).unwrap();

        let conn = PgConnection::establish(&db_addr[..]).unwrap();
        let app_data = web::Data::new(
            AppData{
                pool: Arc::new(pool),
                db_conn: conn
            }
        );

        let app = App::new()
            .app_data(app_data)
            .wrap_fn(
                |req, srv| {
                    let mut passed: bool;
                    if *&req.path().contains("/v1/") {
                        match auth::process_token(&req) {
                            Ok(_token) => passed = true,
                            Err(error) => passed = false
                        }
                    } else {
                        passed = true
                    }

                    if passed {
                        future::Either::Left(srv.call(req).map(|res| res))
                    } else {
                        future::Either::Right(
                            future::ready(
                                Ok(
                                    req.into_response(
                                        HttpResponse::Unauthorized().finish()
                                    )
                                )
                            )
                        )
                    }
                }
            ).wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .configure(view::route_config)
            .default_service(web::route().to(not_found));
        return app
    })
        .bind(&address[..])?
        .run()
        .await

}
