use actix_web::{web, HttpResponse, Responder};
use crate::auth::models::User;
use crate::auth::services;
use sqlx::PgPool;

pub async fn register(db: web::Data<PgPool>, user: web::Json<User>) -> impl Responder {
    match services::register_user(db.get_ref(), &user).await {
        Ok(_) => HttpResponse::Ok().json("Usuario registrado"),
        Err(_) => HttpResponse::InternalServerError().json("Error en el registro"),
    }
}

pub async fn login(db: web::Data<PgPool>, user: web::Json<User>) -> impl Responder {
    match services::login_user(db.get_ref(), &user).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(_) => HttpResponse::Unauthorized().json("Credenciales inválidas"),
    }
}