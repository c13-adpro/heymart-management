use crate::model::supermarket::{CreateSupermarketDto, Supermarket, UpdateSupermarketDto};
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

    pub async fn create(db_pool: PgPool, supermarket: CreateSupermarketDto) -> Option<Supermarket> {
        None
    }

    pub async fn update(
        db_pool: PgPool,
        id: i64,
        supermarket: UpdateSupermarketDto,
    ) -> Option<Supermarket> {
        None
    }

    pub async fn delete(db_pool: PgPool, id: i64) -> Option<Supermarket> {
        None
    }
}
