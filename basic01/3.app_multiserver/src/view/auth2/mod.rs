use actix_web::web;
use actix_web::web::ServiceConfig;
mod login;
mod logout;

pub fn auth_factory(app: &mut ServiceConfig) {
    app.service(
        web::scope("v2")
            .service(
                web::resource("/login")
                    .route(web::get().to(login::login))
            ).service(
            web::resource("/logout")
                .route(web::get().to(logout::logout))
        )
    );
}