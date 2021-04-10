use crate::repository_suite::utils::setup_connection;
use aerodrome_core::store::track::{TrackSqliteRepo, TrackRepository};
use aerodrome_core::model::track::Track;
use chrono::Utc;
use chrono::TimeZone;

#[tokio::test(flavor = "multi_thread")]
async fn track_repo_when_inserted_should_be_fetched() {
    // Given
    let pool = setup_connection().await;
    let track_repo = TrackSqliteRepo::new(pool.clone());

    // When
    let _ = track_repo.save(&(*data::TRACK_ROY_BATTY)).await.unwrap();

    // Then
    let result = track_repo.find_by_id(&data::TRACK_ROY_BATTY.id).await.unwrap();

    assert_eq!(result, *data::TRACK_ROY_BATTY);
}

mod data {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref TRACK_ROY_BATTY: Track = Track {
            id: "b1e376bdf7b14088adfea0a5b276d1b9".to_string(),
            title: "The Ballad of Roy Batty".to_string(),
            artist: "Grumbling Fur".to_string(),
            artist_id: "9bc3c9fcc64d4ce3be7cc9ce0ff856ce".to_string(),
            album: "Glynnaestra".to_string(),
            album_id: "d2c3a9ceeb144d22a8a7af18c36b2bca".to_string(),
            album_artist: "Grumbling Fur".to_string(),
            album_artist_id: "9bc3c9fcc64d4ce3be7cc9ce0ff856ce".to_string(),
            has_cover_art: false,
            track_nr: 4,
            disc_nr: 0,
            disc_subtitle: "".to_string(),
            year: 2013,
            size: 20882994,
            suffix: "flac".to_string(),
            duration: 587.0,
            bitrate: 580,
            genre: "".to_string(),
            full_text: "glynnaestra ballad of roy batty grumbling fur".to_string(),
            compilation: false,
            comment: "".to_string(),
            lyrics: "".to_string(),
            catalog_nr: "".to_string(),
            sort_title: "The Ballad of Roy Batty".to_string(),
            sort_artist_name: "Grumbling Fur".to_string(),
            sort_album_name: "Glynnaestra".to_string(),
            sort_album_artist_name: "Grumbling Fur".to_string(),
            order_artist_name: "Grumbling Fur".to_string(),
            order_album_name: "Glynnaestra".to_string(),
            order_album_artist_name: "Grumbling Fur".to_string(),
            mbz_track_id: "".to_string(),
            mbz_artist_id: "".to_string(),
            mbz_album_id: "".to_string(),
            mbz_album_artist_id: "".to_string(),
            mbz_album_type: "".to_string(),
            mbz_album_comment: "".to_string(),
            path: "/music/Grumbling Fur/2013 - glynnaestra/04 - The Ballad of Roy Batty.flac".to_string(),
            created_at: Utc.ymd(2021, 04, 10).and_hms_milli(2, 21, 40, 555),
            updated_at: Utc.ymd(2021, 04, 10).and_hms_milli(2, 35, 31, 451),
        };
    }
}