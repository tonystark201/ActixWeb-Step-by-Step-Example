use std::sync::Mutex;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, middleware, web, Result};
use actix_web::web::Data;
use log::{debug, info};

async fn index(
    counter_mutex: Data<Mutex<usize>>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    debug!("{req:?}");

    // Increment the counters
    *counter_mutex.lock().unwrap() += 1;

    let body = format!(
        "global mutex counter: {}",
        *counter_mutex.lock().unwrap(),
    );
    Ok(HttpResponse::Ok().body(body))

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("starting HTTP server at http://localhost:8080");

    let counter_mutex = Data::new(Mutex::new(0usize));

    HttpServer::new(move || {
        App::new()
            .app_data(counter_mutex.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/").to(index)
            )

    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}