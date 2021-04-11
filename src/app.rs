use crate::store::album::AlbumSqliteRepo;
use crate::store::track::TrackSqliteRepo;
use crate::web::AerodromeWebServer;
use futures::FutureExt;
use sqlx::SqlitePool;
use tokio::{select, signal};

/// The Aerodrome App runs all required services and background processes
pub struct AerodromeApp {
    web: AerodromeWebServer,
}

impl AerodromeApp {
    pub async fn init() -> Self {
        // TODO: Proper error handling, use database string from configuration
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

        // Repos
        let album_repo = AlbumSqliteRepo::new(pool.clone());
        let track_repo = TrackSqliteRepo::new(pool.clone());

        // Run the migrations
        sqlx::migrate!("db/migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations!");
        // sqlx::migrate!("db/seed")
        //     .run(&pool)
        //     .await
        //     .expect("Failed to run migrations!");

        let web = AerodromeWebServer::init(album_repo, track_repo);

        AerodromeApp { web }
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
