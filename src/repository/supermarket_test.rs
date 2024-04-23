#[cfg(test)]
mod tests {
    use crate::{
        model::supermarket::{CreateSupermarketDto, Supermarket},
        repository::{
            lib::{nuke, setup},
            supermarket::SupermarketRepository,
        },
    };
    use rocket::tokio;

    #[tokio::test]
    async fn test_create_supermarket() {
        let pool = setup().await;
        let mut conn = pool.clone().acquire().await.unwrap();

        let supermarket = SupermarketRepository::create(
            pool.clone(),
            CreateSupermarketDto {
                name: "Supermarket".to_string(),
            },
        )
        .await
        .unwrap();
        assert_eq!(supermarket.name, "Supermarket");
        assert_eq!(supermarket.id, 1);

        let result = sqlx::query!("SELECT id, name, balance FROM supermarket WHERE id = 1")
            .fetch_one(&mut *conn)
            .await
            .unwrap();
        assert_eq!(result.name, "Supermarket");
        assert_eq!(result.id, 1);
        nuke(pool).await;
    }

    #[tokio::test]
    async fn test_get_all_supermarkets() {
        let pool = setup().await;
        let mut conn = pool.clone().acquire().await.unwrap();

        sqlx::query!(
            r#"
            INSERT INTO supermarket (name) VALUES ('Supermarket 1');
            "#,
        )
        .execute(&mut *conn)
        .await
        .unwrap();

        sqlx::query!(
            r#"
            INSERT INTO supermarket (name) VALUES ('Supermarket 2');
            "#,
        )
        .execute(&mut *conn)
        .await
        .unwrap();

        let supermarkets = SupermarketRepository::find_all(pool.clone()).await.unwrap();
        assert_eq!(supermarkets.len(), 2);
        assert_eq!(supermarkets[0].name, "Supermarket 1");
        assert_eq!(supermarkets[1].name, "Supermarket 2");
        nuke(pool).await;
    }
}
