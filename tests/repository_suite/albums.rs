use crate::repository_suite::albums_data::*;
use crate::repository_suite::utils::setup_connection;
use aerodrome_core::store::album::{AlbumRepository, AlbumSqliteRepo};
use futures::Future;
use sqlx::{Pool, Sqlite, SqlitePool};

#[tokio::test(flavor = "multi_thread")]
async fn album_repo_when_inserted_should_be_fetched() {
    // Given
    let mut pool = setup_connection().await;
    let album_repo = AlbumSqliteRepo::new(pool.clone());

    // When
    let id = album_repo.save(&ALBUM_PEPPERS).await.unwrap();

    // Then
    let result = album_repo.find_by_id(id).await.unwrap();

    assert_eq!(result, *ALBUM_PEPPERS);
}
