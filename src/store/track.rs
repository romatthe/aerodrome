use crate::model::track::Track;
use sqlx::SqlitePool;
use sql_builder::SqlBuilder;

#[async_trait::async_trait]
pub trait TrackRepository {
    async fn find_by_id(&self, id: &str) -> anyhow::Result<Track>;
    async fn save(&self, track: &Track) -> anyhow::Result<i64>;
}

pub struct TrackSqliteRepo {
    pool: SqlitePool,
}

impl TrackSqliteRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TrackRepository for TrackSqliteRepo {
    async fn find_by_id(&self, id: &str) -> anyhow::Result<Track> {
        let sql = SqlBuilder::select_from("track")
            .fields(&[
                "id", "title", "artist", "artist_id", "album", "album_id", "album_artist",
                "album_artist_id", "has_cover_art", "track_nr", "disc_nr", "disc_subtitle", "year",
                "size", "suffix", "duration", "bitrate", "genre", "full_text", "compilation",
                "comment", "lyrics", "catalog_nr", "sort_title", "sort_artist_name",
                "sort_album_name", "sort_album_artist_name", "order_artist_name", "order_album_name",
                "order_album_artist_name", "mbz_track_id", "mbz_artist_id", "mbz_album_id",
                "mbz_album_artist_id", "mbz_album_type", "mbz_album_comment", "path", "created_at",
                "updated_at"
            ])
            .and_where_eq("id", "?")
            .sql()?;

        let track: Track = sqlx::query_as(&sql).bind(id).fetch_one(&self.pool).await?;

        Ok(track)
    }

    async fn save(&self, track: &Track) -> anyhow::Result<i64> {
        let sql = SqlBuilder::insert_into("track")
            .fields(&[
                "id", "title", "artist", "artist_id", "album", "album_id", "album_artist",
                "album_artist_id", "has_cover_art", "track_nr", "disc_nr", "disc_subtitle", "year",
                "size", "suffix", "duration", "bitrate", "genre", "full_text", "compilation",
                "comment", "lyrics", "catalog_nr", "sort_title", "sort_artist_name",
                "sort_album_name", "sort_album_artist_name", "order_artist_name", "order_album_name",
                "order_album_artist_name", "mbz_track_id", "mbz_artist_id", "mbz_album_id",
                "mbz_album_artist_id", "mbz_album_type", "mbz_album_comment", "path", "created_at",
                "updated_at"
            ])
            .values(&[
                "?", "?", "?", "?", "?", "?", "?", "?", "?", "?", "?", "?", "?", "?", "?", "?", "?",
                "?", "?", "?", "?", "?", "?", "?", "?", "?", "?", "?", "?", "?", "?", "?", "?", "?",
                "?", "?", "?", "?", "?"
            ])
            .sql()?;

        let id = sqlx::query(&sql)
            .bind(&track.id)
            .bind(&track.title)
            .bind(&track.artist)
            .bind(&track.artist_id)
            .bind(&track.album)
            .bind(&track.album_id)
            .bind(&track.album_artist)
            .bind(&track.album_artist_id)
            .bind(&track.has_cover_art)
            .bind(&track.track_nr)
            .bind(&track.disc_nr)
            .bind(&track.disc_subtitle)
            .bind(&track.year)
            .bind(&track.size)
            .bind(&track.suffix)
            .bind(&track.duration)
            .bind(&track.bitrate)
            .bind(&track.genre)
            .bind(&track.full_text)
            .bind(&track.compilation)
            .bind(&track.comment)
            .bind(&track.lyrics)
            .bind(&track.catalog_nr)
            .bind(&track.sort_title)
            .bind(&track.sort_artist_name)
            .bind(&track.sort_album_name)
            .bind(&track.sort_album_artist_name)
            .bind(&track.order_artist_name)
            .bind(&track.order_album_name)
            .bind(&track.order_album_artist_name)
            .bind(&track.mbz_track_id)
            .bind(&track.mbz_artist_id)
            .bind(&track.mbz_album_id)
            .bind(&track.mbz_album_artist_id)
            .bind(&track.mbz_album_type)
            .bind(&track.mbz_album_comment)
            .bind(&track.path)
            .bind(&track.created_at)
            .bind(&track.updated_at)
            .execute(&self.pool)
            .await?
            .last_insert_rowid();

        Ok(id)
    }
}
