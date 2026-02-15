use axum::{
    routing::{get, post},
    Router, Json
};
use jsonwebtoken::{encode, Header};
use crate::config::get_config;
use crate::structs::{AuthError, Claims, AuthPayload, AuthBody, KEYS};


async fn check_auth(claims: Claims) -> Result<String, AuthError>{
    Ok(format!(
        "You were successfully authenticated! :)\nYour data:\n{claims}",
    ))
}

async fn create_token(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    if payload.client_id != "foo" || payload.client_secret != "bar" {
        tracing::warn!("invalid client id: {}", payload.client_id);
        return Err(AuthError::WrongCredentials);
    }

    let cfg = get_config();

    let claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "Umbrella".to_owned(),
        exp: cfg.jwt_lifetime_seconds()
    };

    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(AuthBody::new(token)))
}

pub fn init_router() -> Router {
    Router::new()
        .route("/check", get(check_auth))
        .route("/authorize", post(create_token))
}