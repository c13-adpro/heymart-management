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
}
