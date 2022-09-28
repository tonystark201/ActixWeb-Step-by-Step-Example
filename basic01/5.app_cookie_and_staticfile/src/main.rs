use actix_files::NamedFile;
use actix_web::{
    get,
    http::{
        header::ContentType,
    },
    cookie::Cookie,
    middleware, web, App, HttpResponse, HttpServer, Responder, Result,
};
use log::{info};


#[get("/index")]
async fn index() -> Result<impl Responder> {
    Ok(NamedFile::open("static/index.html")?)
}

async fn hello() -> Result<impl Responder> {
    let cookie = Cookie::build("token","123123123")
        .domain("127.0.0.1")
        .path("/v1/hello")
        .secure(false)
        .http_only(true)
        .finish();

    let mut response =  HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Hello, world");

    response.add_cookie(&cookie);

    Ok(response)

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("v1")
                    .service(index)
                    .service(
                        web::resource("/hello")
                            .route(web::get().to(hello))
                    )
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}