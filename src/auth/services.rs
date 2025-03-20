use crate::auth::models::{User, Claims};
use sqlx::PgPool;
use jsonwebtoken::{encode, Header, EncodingKey};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn register_user(pool: &PgPool, user: &User) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO users (username, password) VALUES ($1, $2)")
        .bind(&user.username)
        .bind(&user.password)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn login_user(pool: &PgPool, user: &User) -> Result<String, ()> {
    let row: Option<(String,)> = sqlx::query_as("SELECT password FROM users WHERE username = $1")
        .bind(&user.username)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten();

    if let Some((stored_password,)) = row {
        if stored_password == user.password {
            let expiration = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() + 3600; // 1 hora de expiración

            let claims = Claims { sub: user.username.clone(), exp: expiration as usize };
            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(b"secret_key")).unwrap();
            return Ok(token);
        }
    }

    Err(())
}