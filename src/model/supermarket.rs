use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Supermarket {
    pub id: i64,
    pub name: String,
    pub balance: i32,
    pub created_at: String,
}
