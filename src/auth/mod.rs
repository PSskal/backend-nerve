pub mod handlers;
pub mod models;
pub mod services;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/register").route(web::post().to(handlers::register)))
        .service(web::resource("/login").route(web::post().to(handlers::login)));
}