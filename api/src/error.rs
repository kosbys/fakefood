use axum::{http::StatusCode, response::IntoResponse};

pub enum AppError {
    Database(sqlx::Error),
    NotFound,
    BadRequest(String),
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        AppError::Database(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            AppError::BadRequest(s) => (StatusCode::BAD_REQUEST, s),
            AppError::Database(e) => {
                eprintln!("Database encountered an error: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                )
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not Found".to_string()),
        };

        (status, body).into_response()
    }
}
