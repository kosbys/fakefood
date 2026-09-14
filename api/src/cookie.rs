use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use time::SignedDuration;
use uuid::Uuid;

pub fn create_id_cookie(id: Uuid) -> Cookie<'static> {
    Cookie::build(("id", id.to_string()))
        .path("/")
        .http_only(true)
        .max_age(SignedDuration::days(365))
        .same_site(SameSite::Lax)
        .build()
}

pub fn get_id(jar: &CookieJar) -> Option<Uuid> {
    jar.get("id")
        .and_then(|cookie| Uuid::parse_str(cookie.value()).ok())
}
