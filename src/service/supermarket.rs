use crate::{
    lib::heymart_result::Result,
    model::supermarket::{CreateSupermarketDto, UpdateSupermarketDto},
};
use sqlx::PgPool;
pub struct SupermarketService {}

impl SupermarketService {
    pub async fn get_all_supermarkets(db_pool: PgPool) -> Result<()> {
        Ok(())
    }

    pub async fn get_supermarket_by_id(db_pool: PgPool, id: i64) -> Result<()> {
        Ok(())
    }

    pub async fn create_supermarket(
        db_pool: PgPool,
        supermarket: CreateSupermarketDto,
    ) -> Result<()> {
        Ok(())
    }

    pub async fn update_supermarket(
        db_pool: PgPool,
        id: i64,
        supermarket: UpdateSupermarketDto,
    ) -> Result<()> {
        Ok(())
    }

    pub async fn delete_supermarket(db_pool: PgPool, id: i64) -> Result<()> {
        Ok(())
    }
}
