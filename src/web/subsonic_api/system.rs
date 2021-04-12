use crate::web::subsonic_api::{SUBSONIC_XMLNS, SUBSONIC_VERSION};
use lazy_static::lazy_static;
use rocket::get;
use rocket_contrib::xml::Xml;
use serde::Serialize;
use chrono::{Utc, DateTime, TimeZone};

lazy_static! {
    pub static ref EXPIRES: DateTime<Utc> = Utc.ymd(2199, 01, 01).and_hms_milli(0, 00, 00, 000);
}

#[derive(Serialize)]
#[serde(rename = "subsonic-response")]
pub struct PingResponse {
    xmlns: &'static str,
    status: &'static str,
    version: &'static str,
}

impl PingResponse {
    fn new() -> Self {
        Self {
            xmlns: SUBSONIC_XMLNS,
            status: "ok",
            version: SUBSONIC_VERSION,
        }
    }
}

#[derive(Serialize)]
#[serde(rename = "subsonic-response")]
pub struct GetLicenseResponse {
    xmlns: &'static str,
    status: &'static str,
    version: &'static str,
    license: License,
}

#[derive(Serialize)]
#[serde(rename = "subsonic-response")]
#[serde(rename_all = "camelCase")]
struct License {
    valid: bool,
    email: &'static str,
    license_expires: DateTime<Utc>
}

impl GetLicenseResponse {
    pub fn new() -> Self {
        Self {
            xmlns: SUBSONIC_XMLNS,
            status: "ok",
            version: SUBSONIC_VERSION,
            license: License {
                valid: true,
                email: "aerodrome@github.com",
                license_expires: *EXPIRES
            }
        }
    }
}

# [get("/ping")]
pub fn ping() -> Xml<PingResponse> {
    Xml(PingResponse::new())
}

# [get("/getLicense")]
pub fn get_license() -> Xml<GetLicenseResponse> {
    Xml(GetLicenseResponse::new())
}

