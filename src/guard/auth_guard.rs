use std::env;

use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    serde::{Deserialize, Serialize},
    Request,
};

use crate::library::{error_response::ErrorResponse, reqwest_client::REQWEST_CLIENT};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(crate = "rocket::serde")]
pub enum Role {
    ADMIN,
    CUSTOMER,
    MANAGER,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: Role,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct UserResponse {
    id: i64,
    username: String,
    email: String,
    role: String,
}

#[derive(Debug)]
pub struct AuthGuard {
    pub user: User,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = ErrorResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ErrorResponse> {
        let auth_header = req.headers().get_one("Authorization");
        if auth_header.is_none() {
            return Outcome::Error((
                Status::Unauthorized,
                ErrorResponse {
                    status_code: rocket::http::Status::Unauthorized,
                    message: "Unauthorized".to_string(),
                },
            ));
        }
        let token = auth_header.unwrap().split_whitespace().last().unwrap();
        let auth_url = env::var("AUTH_SERVICE_URL").expect("AUTH_SERVICE_URL must be set.");
        let request_url = format!("{}/users/me", auth_url);
        let response = REQWEST_CLIENT
            .get(&request_url)
            .bearer_auth(token)
            .send()
            .await;
        if response.is_err() {
            return Outcome::Error((
                Status::Unauthorized,
                ErrorResponse {
                    status_code: rocket::http::Status::Unauthorized,
                    message: "Unauthorized, unable to ping the auth service".to_string(),
                },
            ));
        }
        let response = response.unwrap();
        if !response.status().is_success() {
            return Outcome::Error((
                Status::Unauthorized,
                ErrorResponse {
                    status_code: rocket::http::Status::Unauthorized,
                    message: "Unauthorized, unable to contact the auth service".to_string(),
                },
            ));
        }

        let data = match response.json::<UserResponse>().await {
            Ok(data) => data,
            Err(_) => {
                return Outcome::Error((
                    Status::Unauthorized,
                    ErrorResponse {
                        status_code: rocket::http::Status::Unauthorized,
                        message: "Unauthorized, your token has expired or invalid".to_string(),
                    },
                ))
            }
        };

        Outcome::Success(AuthGuard {
            user: User {
                id: data.id as i32,
                username: data.username,
                email: data.email,
                role: match data.role.as_str() {
                    "ADMIN" => Role::ADMIN,
                    "CUSTOMER" => Role::CUSTOMER,
                    "MANAGER" => Role::MANAGER,
                    _ => Role::CUSTOMER,
                },
            },
        })
    }
}
