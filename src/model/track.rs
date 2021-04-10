use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Track {
    pub id: String,
    pub title: String,

    // Artist
    pub artist: String,
    pub artist_id: String,

    // Album
    pub album: String,
    pub album_id: String,
    pub album_artist: String,
    pub album_artist_id: String,
    pub has_cover_art: bool,
    pub track_nr: u32,
    pub disc_nr: u32,
    pub disc_subtitle: String,

    pub year: u32,
    // TODO: u64 is represented as lossy f64 under the hood in SQLite, therefore SQLx does not support
    // `Decode` and `Encode` on Rust u64 types for SQLite. This means that the max size is fixed
    // u32 currently, which is pretty large for a media file, but it might be good think this over
    pub size: u32,
    pub suffix: String,
    pub duration: f32,
    pub bitrate: u32,
    pub genre: String,
    pub full_text: String,
    pub compilation: bool,
    pub comment: String,
    pub lyrics: String,
    pub catalog_nr: String,

    // Sorting
    pub sort_title: String,
    pub sort_artist_name: String,
    pub sort_album_name: String,
    pub sort_album_artist_name: String,

    // Ordering
    pub order_artist_name: String,
    pub order_album_name: String,
    pub order_album_artist_name: String,

    // MusicBrainz
    pub mbz_track_id: String,
    pub mbz_artist_id: String,
    pub mbz_album_id: String,
    pub mbz_album_artist_id: String,
    pub mbz_album_type: String,
    pub mbz_album_comment: String,

    // Meta
    pub path: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
