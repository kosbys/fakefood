use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use crate::{cookie::get_id, error::AppError};

pub struct CurrentUser {
    pub id: Uuid,
}

impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);

        let id = get_id(&jar)
            .ok_or_else(|| AppError::BadRequest("Missing or invalid cookie".to_string()))?;

        Ok(CurrentUser { id })
    }
}
