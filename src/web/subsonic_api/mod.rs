use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};
use crate::store::track::{TrackSqliteRepo, TrackRepository};
use rocket::response::content::Xml;
use crate::model::track::Track;
use rocket::response::status::NotFound;
use rocket::{get, State};

#[derive(Serialize)]
#[serde(rename = "subsonic-response")]
pub struct ErrorResponse {
    xmlns: String,
    status: String,
    version: String,
    error: Error,
}

impl ErrorResponse {
    fn new(code: u32, message: &str) -> Self {
        ErrorResponse {
            xmlns: "http://subsonic.org/restapi".to_owned(),
            status: "failed".to_owned(),
            version: "1.8.0".to_owned(),
            error: Error {
                code,
                message: message.to_owned()
            }
        }
    }
}

#[derive(Serialize)]
#[serde(rename = "error")]
pub struct Error {
    code: u32,
    message: String,
}

#[derive(Serialize)]
#[serde(rename = "subsonic-response")]
pub struct GetSongResponse {
    xmlns: String,
    status: String,
    version: String,
    song: Child,
}

// type Child struct {
//     Id                    string     `xml:"id,attr"                                 json:"id"`
//     Parent                string     `xml:"parent,attr,omitempty"                   json:"parent,omitempty"`
//     IsDir                 bool       `xml:"isDir,attr"                              json:"isDir"`
//     Title                 string     `xml:"title,attr,omitempty"                    json:"title,omitempty"`
//     Name                  string     `xml:"name,attr,omitempty"                     json:"name,omitempty"`
//     Album                 string     `xml:"album,attr,omitempty"                    json:"album,omitempty"`
//     Artist                string     `xml:"artist,attr,omitempty"                   json:"artist,omitempty"`
//     Track                 int        `xml:"track,attr,omitempty"                    json:"track,omitempty"`
//     Year                  int        `xml:"year,attr,omitempty"                     json:"year,omitempty"`
//     Genre                 string     `xml:"genre,attr,omitempty"                    json:"genre,omitempty"`
//     CoverArt              string     `xml:"coverArt,attr,omitempty"                 json:"coverArt,omitempty"`
//     Size                  int64      `xml:"size,attr,omitempty"                     json:"size,omitempty"`
//     ContentType           string     `xml:"contentType,attr,omitempty"              json:"contentType,omitempty"`
//     Suffix                string     `xml:"suffix,attr,omitempty"                   json:"suffix,omitempty"`
//     Starred               *time.Time `xml:"starred,attr,omitempty"                  json:"starred,omitempty"`
//     TranscodedContentType string     `xml:"transcodedContentType,attr,omitempty"    json:"transcodedContentType,omitempty"`
//     TranscodedSuffix      string     `xml:"transcodedSuffix,attr,omitempty"         json:"transcodedSuffix,omitempty"`
//     Duration              int        `xml:"duration,attr,omitempty"                 json:"duration,omitempty"`
//     BitRate               int        `xml:"bitRate,attr,omitempty"                  json:"bitRate,omitempty"`
//     Path                  string     `xml:"path,attr,omitempty"                     json:"path,omitempty"`
//     PlayCount             int64      `xml:"playCount,attr,omitempty"                json:"playcount,omitempty"`
//     DiscNumber            int        `xml:"discNumber,attr,omitempty"               json:"discNumber,omitempty"`
//     Created               *time.Time `xml:"created,attr,omitempty"                  json:"created,omitempty"`
//     AlbumId               string     `xml:"albumId,attr,omitempty"                  json:"albumId,omitempty"`
//     ArtistId              string     `xml:"artistId,attr,omitempty"                 json:"artistId,omitempty"`
//     Type                  string     `xml:"type,attr,omitempty"                     json:"type,omitempty"`
//     UserRating            int        `xml:"userRating,attr,omitempty"               json:"userRating,omitempty"`
//     SongCount             int        `xml:"songCount,attr,omitempty"                json:"songCount,omitempty"`
//     IsVideo               bool       `xml:"isVideo,attr"                            json:"isVideo"`
//     BookmarkPosition      int64      `xml:"bookmarkPosition,attr,omitempty"         json:"bookmarkPosition,omitempty"`
//     /*
//        <xs:attribute name="averageRating" type="sub:AverageRating" use="optional"/>  <!-- Added in 1.6.0 -->
//     */
// }

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
    user_rating:  Option<u32>,
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
pub async fn get_song(id: &str, tracks: State<'_, TrackSqliteRepo>) -> Result<Xml<String>, NotFound<Xml<String>>> {
    let test = Child {
        id: "JoeBiden".to_string(),
        parent: None,
        is_dir: false,
        title: "JoeBiden Returns".to_string(),
        album: None,
        artist: None,
        track: None,
        year: None,
        genre: None,
        covert_art: None,
        size: None,
        content_type: None,
        suffix: None,
        transcoded_content_type: None,
        transcoded_suffix: None,
        duration: None,
        bit_rate: None,
        path: None,
        is_video: None,
        user_rating: None,
        average_rating: None,
        play_count: None,
        disc_number: None,
        created: None,
        starred: None,
        album_id: None,
        artist_id: None,
        r#type: None,
        bookmark_position: None,
        original_width: None,
        original_height: None
    };

    let y = GetSongResponse {
        xmlns: "http://subsonic.org/restapi".to_owned(),
        status: "ok".to_string(),
        version: "1.8.1".to_string(),
        song: test,
    };

    tracks
        .find_by_id(id)
        .await
        .map(|a| Xml(quick_xml::se::to_string(&y).unwrap()))
        .map_err(|_| {
            let error = ErrorResponse::new(70, "No track with id {} found!");
            NotFound(Xml(quick_xml::se::to_string(&error).unwrap()))
        })
}