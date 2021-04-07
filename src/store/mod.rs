pub(crate) mod album;

use sqlx::{Database, Pool};

pub trait Repository<T, D: Database>: Send + Sync {
    fn new(pool: Pool<D>) -> Self;
}

