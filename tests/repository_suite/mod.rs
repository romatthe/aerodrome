mod albums;
mod albums_data;

mod utils {
    use sqlx::SqlitePool;

    pub async fn setup_connection() -> SqlitePool {
        let mut pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

        // Run the migrations
        sqlx::migrate!("db/migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations!");

        pool
    }
}
