use std::env;

use axum::{Json, http::StatusCode};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use serde::Deserialize;

use crate::error::AppError::{self};

#[derive(Debug, Deserialize)]
pub struct AdminKey {
    pub key: String,
}

pub async fn admin_login(
    jar: CookieJar,
    Json(body): Json<AdminKey>,
) -> Result<(CookieJar, StatusCode), AppError> {
    let admin_key = env::var("ADMIN_KEY").expect("Missing admin key");

    if body.key != admin_key {
        return Err(AppError::Forbidden);
    }

    let cookie = Cookie::build(("admin", admin_key))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();

    Ok((jar.add(cookie), StatusCode::OK))
}
