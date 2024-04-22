use rocket::tokio;
use std::env;

#[macro_use]
extern crate rocket;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .unwrap();

    rocket::build()
        .manage(pool)
        .mount("/", routes![])
        .launch()
        .await
        .unwrap();
}
