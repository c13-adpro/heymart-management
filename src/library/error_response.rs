use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    pub status_code: Status,
    pub message: String,
}

pub fn error_response(status_code: Status, message: String) -> Custom<Json<ErrorResponse>> {
    return Custom(
        status_code,
        Json::from(ErrorResponse {
            status_code: status_code,
            message: message,
        }),
    );
}
