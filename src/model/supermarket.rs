use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Supermarket {
    pub id: i64,
    pub name: String,
    pub balance: i32,
    pub created_at: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateSupermarketDto {
    pub name: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateSupermarketDto {
    pub name: Option<String>,
    pub balance: Option<i32>,
}
