use std::env;
use actix_web::web::ServiceConfig;
mod auth1;
mod auth2;

pub fn factory1(app: &mut ServiceConfig) {
    auth1::auth_factory(app);
}

pub fn factory2(app: &mut ServiceConfig) {
    auth2::auth_factory(app);
}