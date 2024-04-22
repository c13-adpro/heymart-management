use chrono::NaiveDateTime;

pub struct Supermarket {
    pub id: i64,
    pub name: String,
    pub balance: i32,
    pub created_at: chrono::NaiveDateTime,
}
