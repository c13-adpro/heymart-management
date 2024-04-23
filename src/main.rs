#[macro_use]
extern crate rocket;
pub mod controller;
pub mod library;
pub mod model;
pub mod repository;
pub mod service;

use controller::route_stage;
use dotenvy::dotenv;
use rocket_cors::AllowedOrigins;
use shuttle_rocket::ShuttleRocket;
use sqlx::postgres::PgPoolOptions;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore) -> ShuttleRocket {
    let db_url = secrets.get("DATABASE_URL").unwrap();
    secrets.into_iter().for_each(|(key, value)| {
        std::env::set_var(key, value);
    });

    dotenv().ok();
    println!("DATABASE_URL: {}", db_url);

    let pool = PgPoolOptions::new()
        // .max_connections(10)
        .connect(&db_url)
        .await
        .unwrap();

    let allowed_origins = AllowedOrigins::some_exact(&["*"]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    let server = rocket::build()
        .manage(pool)
        .attach(cors)
        .attach(route_stage());

    Ok(server.into())
}
