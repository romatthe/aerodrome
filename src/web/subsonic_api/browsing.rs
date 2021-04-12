use crate::model::track::Track;
use crate::store::track::{TrackRepository, TrackSqliteRepo};
use crate::web::subsonic_api::{ErrorResponse, SubsonicError, SUBSONIC_VERSION, SUBSONIC_XMLNS};
use chrono::{DateTime, Utc};
use mime_guess::MimeGuess;
use rocket::response::status::NotFound;
use rocket::{get, State};
use rocket_contrib::xml::Xml;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename = "subsonic-response")]
pub struct GetSongResponse {
    xmlns: &'static str,
    status: &'static str,
    version: &'static str,
    song: Child,
}

impl GetSongResponse {
    fn new(song: Child) -> Self {
        Self {
            xmlns: SUBSONIC_XMLNS,
            status: "ok",
            version: SUBSONIC_VERSION,
            song,
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Child {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<String>,
    is_dir: bool,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    album: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    artist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    track: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    genre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    covert_art: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transcoded_content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transcoded_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bit_rate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_video: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_rating: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    average_rating: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    play_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disc_number: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starred: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    album_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    artist_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bookmark_position: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    original_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    original_height: Option<u32>,
}

#[get("/getSong/<id>")]
pub async fn get_song(
    id: &str,
    tracks: State<'_, TrackSqliteRepo>,
) -> Result<Xml<GetSongResponse>, NotFound<Xml<ErrorResponse>>> {
    tracks
        .find_by_id(id)
        .await
        .map(|track| Xml(GetSongResponse::from(track)))
        .map_err(|_| {
            let error = ErrorResponse::from(
                SubsonicError::DataNotFound,
                &format!("No track with id `{}` found!", id),
            );
            NotFound(Xml(error))
        })
}

impl From<Track> for GetSongResponse {
    fn from(track: Track) -> Self {
        let c = Child {
            id: track.id.clone(),
            parent: Some(track.album_id.clone()),
            is_dir: false,
            title: track.title,
            album: Some(track.album),
            artist: Some(track.artist),
            track: Some(track.track_nr),
            year: Some(track.year),
            genre: Some(track.genre),
            size: Some(track.size as u64),
            content_type: Some(extension_to_mime(&track.suffix)),
            suffix: Some(track.suffix),
            // TODO: Transcoding info comes from the request, that's for later
            transcoded_content_type: None,
            // TODO: Transcoding info comes from the request, that's for later
            transcoded_suffix: None,
            duration: Some(track.duration as u32),
            bit_rate: Some(track.bitrate),
            // TODO: Figure out what the hell `path` is
            path: None,
            is_video: Some(false),
            // TODO: `user_rating` should be a Track annotation
            user_rating: None,
            average_rating: None,
            // TODO: `play_count` should be a Track annotation
            play_count: None,
            // TODO: This should be `None` if disc_nr is `0`
            disc_number: Some(track.disc_nr),
            created: Some(track.created_at),
            // TODO: `starred_at` should be a Track annotation
            starred: None,
            album_id: Some(track.album_id.clone()),
            artist_id: Some(track.artist_id),
            r#type: Some("music".to_owned()),
            // TODO: `bookmark_position` should be a Track annotation
            bookmark_position: None,

            // TODO: Are these needed?
            original_width: None,
            original_height: None,

            covert_art: if track.has_cover_art {
                Some(track.id)
            } else {
                Some("al-".to_owned() + &track.album_id)
            },
        };

        GetSongResponse::new(c)
    }
}

#[inline]
pub fn extension_to_mime(ext: &str) -> String {
    MimeGuess::from_ext(ext)
        .first_raw()
        .unwrap_or(ext)
        .to_owned()
}
