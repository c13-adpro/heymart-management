use crate::model::supermarket::Supermarket;
use rocket::tokio;
use sqlx::PgPool;

pub struct SupermarketRepository {}

impl SupermarketRepository {
    pub async fn find_all(db_pool: PgPool) -> Option<Vec<Supermarket>> {
        None
    }

    pub async fn find_by_id(db_pool: PgPool, id: i64) -> Option<Supermarket> {
        None
    }

    pub async fn create(db_pool: PgPool, supermarket: Supermarket) -> Option<Supermarket> {
        None
    }

    pub async fn update(db_pool: PgPool, id: i64, supermarket: Supermarket) -> Option<Supermarket> {
        None
    }

    pub async fn delete(db_pool: PgPool, id: i64) -> Option<Supermarket> {
        None
    }
}
