use axum::{
    extract::Request,
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
