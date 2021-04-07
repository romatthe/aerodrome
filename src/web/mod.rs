mod aerodrome_api;
mod subsonic_api;

pub struct AerodromeWebServer {

}

impl AerodromeWebServer {
    pub fn init() -> Self {
        AerodromeWebServer {

        }
    }

    pub fn run(self) -> rocket::Rocket {
        rocket::ignite()
            .mount("/hello", rocket::routes![aerodrome_api::world])
    }
}