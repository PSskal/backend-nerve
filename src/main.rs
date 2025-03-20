mod auth;
mod websockets;

use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no encontrado");
    let pool = PgPool::connect(&database_url).await.expect("Error en conexión DB");

    println!("🚀 Servidor corriendo en http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(auth::config)
            .configure(websockets::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}