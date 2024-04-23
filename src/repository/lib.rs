use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn setup() -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(5) // Adjust as needed
        .connect("postgresql://juankhusuma:ob9Ng3USaFYq@ep-summer-cell-39629429.ap-southeast-1.aws.neon.tech/test_adpro?sslmode=require")
        .await
        .expect("Failed to create pool");
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    return pool;
}

pub async fn nuke(pool: PgPool) {
    sqlx::query("DROP SCHEMA public CASCADE; CREATE SCHEMA public;")
        .execute(&pool)
        .await
        .unwrap();
}
