use sqlx::{Pool, Sqlite, SqlitePool};
use aerodrome_core::store::Repository;
use aerodrome_core::store::album::AlbumRepository;
use futures::Future;

#[tokio::test]
async fn test() {
    let albums = utils::setup_connection().await;


}

mod utils {
    use super::*;

    pub async fn setup_connection() -> AlbumRepository<Sqlite> {
        let mut pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        let album_repo = AlbumRepository::new(pool.clone());

        // Run the migrations
        sqlx::migrate!("db/migrations").run(&pool).await.expect("Failed to run migrations!");
        // Insert test data
        sqlx::migrate!("db/tests").run(&pool).await.expect("Failed to run migrations!");

        album_repo
    }
}