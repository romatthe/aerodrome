use sqlx::SqlitePool;
use crate::model::album::Album;
use sql_builder::SqlBuilder;

#[async_trait::async_trait]
pub trait AlbumRepository {
    async fn find_by_id(&self, id: i64) -> anyhow::Result<Album>;
    async fn save(&self, album: &Album) -> anyhow::Result<i64>;
}

pub struct AlbumSqliteRepo {
    pool: SqlitePool
}

impl AlbumSqliteRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool
        }
    }
}

#[async_trait::async_trait]
impl AlbumRepository for AlbumSqliteRepo {
    async fn find_by_id(&self, id: i64) -> anyhow::Result<Album> {
        let sql = SqlBuilder::select_from("album")
            .fields(&[ "id", "name", "artist" ])
            .and_where_eq("id", "?")
            .sql()?;

        let album: Album= sqlx::query_as(&sql)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(album)
    }

    async fn save(&self, album: &Album) -> anyhow::Result<i64> {
        let sql = SqlBuilder::insert_into("album")
            .fields(&[ "name", "artist" ])
            .values(&[ "?", "?" ])
            .sql()?;

        let id = sqlx::query(&sql)
            .bind(&album.name)
            .bind(&album.artist)
            .execute(&self.pool)
            .await?
            .last_insert_rowid();

        Ok(id)
    }
}