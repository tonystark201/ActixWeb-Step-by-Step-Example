use std::env;
use actix_web::web::ServiceConfig;
mod auth;
mod index;

pub fn views_factory(app: &mut ServiceConfig) {
    auth::auth_factory(app);
    index::index_factory(app);
}