use futures::FutureExt;
use tokio::{signal, select};
use crate::web::AerodromeWebServer;
use sqlx::{Acquire, SqlitePool, Database, Pool};
use crate::store::album::AlbumRepository;
use crate::store::Repository;
use sqlx::migrate::Migrate;

/// The Aerodrome App runs all required services and background processes
pub struct AerodromeApp<D: Database> {
    web: AerodromeWebServer<D>,
}

impl <D: Database> AerodromeApp<D> {
    pub async fn init() -> Self
        where <D as sqlx::Database>::Connection: sqlx::migrate::Migrate
    {
        // TODO: Proper error handling, use database string from configuration
        let mut pool = Pool::connect("sqlite::memory:").await.unwrap();

        // Repos
        let album_repo = AlbumRepository::new(pool.clone());

        // Run the migrations
        sqlx::migrate!("db/migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations!");

        let web = AerodromeWebServer::init(album_repo);

        AerodromeApp {
            web
        }
    }

    pub async fn run(self) {
        let task_web = self.web.run().launch().fuse();
        let task_signal = signal::ctrl_c().fuse();

        // Wait for either process to exit
        select! {
            _ = task_web => { },
            _ = task_signal => println!("Shutting down Aerodrome! 👋️"),
        }
    }
}