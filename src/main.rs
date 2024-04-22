use rocket::tokio;

#[macro_use]
extern crate rocket;

#[tokio::main]
async fn main() {
    rocket::build()
        .mount("/", routes![])
        .launch()
        .await
        .unwrap();
}
