use actix_web::web;
use actix_web::web::ServiceConfig;
mod auth;
mod students;
mod teachers;
mod home;
mod users;


pub fn route_config(app: &mut ServiceConfig) {
    app.service(
        web::scope("v1").service(
            web::resource("/users").route(
                web::to(users::users)
            )
        ).service(
            web::resource("/students").route(
                web::to(students::students)
            )
        ).service(
            web::resource("/student/{uid}").route(
                web::to(students::student)
            )
        ).service(
            web::resource("/teachers").route(
                web::to(teachers::teachers)
            )
        ).service(
            web::resource("/teacher/{uid}").route(
                web::to(teachers::teacher)
            )
        )
    ).service(
        web::scope("auth").service(
            web::resource("/login")
                .route(web::to(auth::login::login))
        ).service(
            web::resource("/logout")
                .route(web::to(auth::logout::logout))
        )
    ).service(
        web::resource("/")
            .route(web::get().to(home::index::home))
    ).service(
        web::resource("/login")
            .route(web::get().to(home::login::login))
    ).service(
        web::resource("/logout")
            .route(web::get().to(home::logout::logout))
    );
}
