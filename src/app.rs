use futures::FutureExt;
use tokio::{signal, select};
use crate::web::AerodromeWebServer;

/// The Aerodrome App runs all required services and background processes
pub struct AerodromeApp {
    web: AerodromeWebServer
}

impl AerodromeApp {
    pub fn new() -> Self {
        AerodromeApp {
            web: AerodromeWebServer { }
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