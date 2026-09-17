use std::env;

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use crate::cookie::{create_id_cookie, get_id};

pub async fn id_cookie(jar: CookieJar, req: Request, next: Next) -> Response {
    let j = if get_id(&jar).is_none() {
        let id = Uuid::new_v4();
        jar.add(create_id_cookie(id))
    } else {
        jar
    };

    let res = next.run(req).await;

    (j, res).into_response()
}

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
