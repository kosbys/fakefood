use std::env;

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;

pub async fn admin(jar: CookieJar, req: Request, next: Next) -> Response {
    let admin_secret = env::var("ADMIN_KEY").expect("Missing admin key");

    let is_admin = jar
        .get("admin_key")
        .map(|cookie| cookie.value() == admin_secret)
        .unwrap_or(false);

    if !is_admin {
        return StatusCode::FORBIDDEN.into_response();
    }

    next.run(req).await
}
