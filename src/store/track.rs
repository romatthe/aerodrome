use crate::model::track::Track;
use sqlx::SqlitePool;
use sql_builder::SqlBuilder;

// time="2021-04-09T21:36:06Z" level=trace msg="SQL: `SELECT starred, starred_at, play_count, play_date, rating, * FROM album LEFT JOIN annotation on (annotation.item_id = album.id AND annotation.item_type = 'album' AND annotation.user_id = '626cd7ca-0f85-4cfb-af7a-556dd16f3342') ORDER BY order_album_name asc, order_album_artist_name asc LIMIT 18`" args="[]" elapsedTime="174.25µs" requestId=d4031c2cd9dd/1DlGRcnzrS-000014 rowsAffected=1
// SELECT starred, starred_at, play_count, play_date, rating, *
// FROM album
// LEFT JOIN annotation on (annotation.item_id = album.id
//  AND annotation.item_type = 'album' AND annotation.user_id = '626cd7ca-0f85-4cfb-af7a-556dd16f3342')
// ORDER BY order_album_name asc, order_album_artist_name asc LIMIT 18

/// QUERY THE TRACKS IN AN ALBUM
// SELECT
//  starred, starred_at, play_count, play_date, rating, media_file.*, position as bookmark_position
// FROM media_file
// LEFT JOIN annotation on (
//  annotation.item_id = media_file.id
//  AND annotation.item_type = 'media_file'
//  AND annotation.user_id = '626cd7ca-0f85-4cfb-af7a-556dd16f3342'
// )
// LEFT JOIN bookmark on (
//  bookmark.item_id = media_file.id
//  AND bookmark.item_type = 'media_file'
//  AND bookmark.user_id = '626cd7ca-0f85-4cfb-af7a-556dd16f3342')
// WHERE (album_id = ?)
// ORDER BY order_album_name asc, disc_number asc, track_number asc, order_artist_name asc, title asc`

#[async_trait::async_trait]
pub trait AlbumRepository {
    async fn find_by_id(&self, id: i64) -> anyhow::Result<Track>;
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
impl AlbumRepository for TrackSqliteRepo {
    async fn find_by_id(&self, id: i64) -> anyhow::Result<Track> {
        let sql = SqlBuilder::select_from("track")
            .fields(&["*"])
            .and_where_eq("id", "?")
            .sql()?;

        let track: Track = sqlx::query_as(&sql).bind(id).fetch_one(&self.pool).await?;

        Ok(track)
    }
}
