use sqlx::{Database, Pool};
use crate::model::album::Album;
use crate::store::Repository;

pub struct AlbumRepository<D: Database> {
    pool: Pool<D>
}

impl <D: Database> Repository<Album, D> for AlbumRepository<D> {
    fn new(pool: Pool<D>) -> Self {
        Self {
            pool
        }
    }
}