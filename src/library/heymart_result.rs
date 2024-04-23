use rocket::{response::status::Custom, serde::json::Json};

use super::error_response::ErrorResponse;

pub type Error = Custom<Json<ErrorResponse>>;
pub type Result<T, E = Error> = std::result::Result<T, E>;
