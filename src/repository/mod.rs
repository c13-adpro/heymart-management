pub mod supermarket;

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use sqlx::{postgres::PgPoolOptions, PgPool};

    use crate::model::supermarket::{CreateSupermarketDto, Supermarket};

    use super::supermarket::SupermarketRepository;

    #[sqlx::test]
    async fn test_create_supermarket(pool: PgPool) {
        let supermarket = SupermarketRepository::create(
            pool,
            CreateSupermarketDto {
                name: "Supermarket".to_string(),
            },
        )
        .await
        .unwrap();
        assert_eq!(supermarket.name, "Supermarket");
        assert_eq!(supermarket.id, 1);

        let mut conn = pool.acquire().await.unwrap();
        let result = sqlx::query!("INSERT INTO supermarket (name) VALUES ('test')",)
            .fetch_one(&mut *conn)
            .await
            .unwrap();
        assert_eq!(result.name, "Supermarket");
        assert_eq!(result.id, 1);
    }
}
