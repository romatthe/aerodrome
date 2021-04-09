use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Track {
    id: String,
    title: String,

    // Artist
    artist: String,
    artist_id: String,

    // Album
    album: String,
    album_id: String,
    album_artist: String,
    album_artist_id: String,
    has_cover_art: bool,
    track_nr: u32,
    disc_nr: u32,
    disc_subtitle: String,

    year: u32,
    // TODO: u64 is represented as lossy f64 under the hood in SQLite, therefore SQLx does not support
    // `Decode` and `Encode` on Rust u64 types for SQLite. This means that the max size is fixed
    // u32 currently, which is pretty large for a media file, but it might be good think this over
    size: u32,
    suffix: String,
    duration: f32,
    bitrate: u32,
    genre: String,
    full_text: String,
    compilation: bool,
    comment: String,
    lyrics: String,
    catalog_nr: String,

    // Sorting
    sort_title: String,
    sort_artist_name: String,
    sort_album_name: String,
    sort_album_artist_name: String,

    // Ordering
    order_artist_name: String,
    order_album_name: String,
    order_album_artist_name: String,

    // MusicBrainz
    mbz_track_id: String,
    mbz_artist_id: String,
    mbz_album_id: String,
    mbz_album_artist_id: String,
    mbz_album_type: String,
    mbz_album_comment: String,

    // Meta
    path: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
