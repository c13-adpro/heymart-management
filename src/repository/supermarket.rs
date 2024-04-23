use crate::model::supermarket::{CreateSupermarketDto, Supermarket, UpdateSupermarketDto};
use sqlx::{query, PgPool};

pub struct SupermarketRepository {}

impl SupermarketRepository {
    pub async fn find_all(db_pool: PgPool) -> Option<Vec<Supermarket>> {
        let query = sqlx::query_as!(
            Supermarket,
            r#"
            SELECT id, name, balance, created_at::text
            FROM supermarket
            "#,
        )
        .fetch_all(&db_pool)
        .await;

        match query {
            Ok(supermarkets) => Some(supermarkets),
            Err(_) => None,
        }
    }

    pub async fn find_by_id(db_pool: PgPool, id: i64) -> Option<Supermarket> {
        let query = sqlx::query_as!(
            Supermarket,
            r#"
            SELECT id, name, balance, created_at::text
            FROM supermarket
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&db_pool)
        .await;

        match query {
            Ok(supermarket) => Some(supermarket),
            Err(_) => None,
        }
    }

    pub async fn create(db_pool: PgPool, supermarket: CreateSupermarketDto) -> Option<Supermarket> {
        let query = sqlx::query_as!(
            Supermarket,
            "INSERT INTO supermarket (name) VALUES ($1) RETURNING id, name, balance, created_at::text",
            supermarket.name
        )
        .fetch_one(&db_pool)
        .await;

        match query {
            Ok(supermarket) => Some(supermarket),
            Err(_) => None,
        }
    }

    pub async fn update(
        db_pool: PgPool,
        id: i64,
        supermarket: UpdateSupermarketDto,
    ) -> Option<Supermarket> {
        let query = sqlx::query_as!(
            Supermarket,
            r#"
            UPDATE supermarket
            SET name = COALESCE($2, name),
                balance = COALESCE($3, balance)
            WHERE id = $1
            RETURNING id, name, balance, created_at::text
            "#,
            id,
            supermarket.name,
            supermarket.balance
        )
        .fetch_one(&db_pool)
        .await;

        match query {
            Ok(supermarket) => Some(supermarket),
            Err(_) => None,
        }
    }

    pub async fn delete(db_pool: PgPool, id: i64) -> Option<Supermarket> {
        None
    }
}
