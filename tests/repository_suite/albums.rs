use aerodrome_core::store::album::{AlbumRepository, AlbumSqliteRepo};
use aerodrome_core::model::album::Album;
use crate::repository_suite::utils::setup_connection;

#[tokio::test(flavor = "multi_thread")]
async fn album_repo_when_inserted_should_be_fetched() {
    // Given
    let pool = setup_connection().await;
    let album_repo = AlbumSqliteRepo::new(pool.clone());

    // When
    let _ = album_repo.save(&(*data::ALBUM_THRILLER)).await.unwrap();

    // Then
    let result = album_repo.find_by_id(&data::ALBUM_THRILLER.id).await.unwrap();

    assert_eq!(result, *data::ALBUM_THRILLER);
}

mod data {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref ALBUM_THRILLER: Album = Album {
            id: "364baec7e83c45ae93949bccad11998d".to_owned(),
            name: "Thriller".to_owned(),
            artist: "Michael Jackson".to_owned()
        };
        pub static ref ALBUM_BACK_IN_BLACK: Album = Album {
            id: "ab1120fcbfde493fadd5-56889156bccf".to_owned(),
            name: "AC/DC".to_owned(),
            artist: "Back in Black".to_owned()
        };
        pub static ref ALBUM_DARK_SIDE: Album = Album {
            id: "96ed931808734e1ca71b9ef80aedb7b5".to_owned(),
            name: "Pink Floyd".to_owned(),
            artist: "Dark Side of the Moon".to_owned()
        };
    }
}