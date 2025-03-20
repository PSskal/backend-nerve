pub mod handlers;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ws").route(web::get().to(handlers::ws_handler)));
}