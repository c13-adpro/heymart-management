use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Supermarket {
    pub id: i64,
    pub name: String,
    pub balance: i32,
    pub created_at: Option<String>,
    pub manager_id: i32,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateSupermarketDto {
    pub manager_id: i32,
    pub name: String,
}

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UpdateSupermarketDto {
    pub name: Option<String>,
    pub balance: Option<i32>,
    pub manager_id: Option<i32>,
}
