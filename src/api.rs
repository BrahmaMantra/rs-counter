use axum::{http::StatusCode, response::IntoResponse};

pub mod usr;
pub enum ApiError{
    SqlxError(sqlx::Error),
    OtherError(String),
}

impl IntoResponse for ApiError{
    fn into_response(self) -> axum::response::Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        ApiError::SqlxError(error)
    }
}

// impl From<String> for ApiError {
//     fn from(error: String) -> Self {
//         ApiError::OtherError(error)
//     }
// }