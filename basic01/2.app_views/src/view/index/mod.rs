use actix_web::web;
use actix_web::web::ServiceConfig;
mod index;

pub fn index_factory(app: &mut ServiceConfig) {
    app.service(
        web::resource("/")
            .route(web::get().to(index::index))
    );
}