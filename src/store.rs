use chrono::NaiveDateTime;
use serde::{Serialize};

use crate::wialon::ShortDataPacket;

#[derive(Serialize, Debug)]
pub struct GeoPacket {
    imei: String,
    timestamp: NaiveDateTime,
    lat: f64,
    lon: f64,
    speed: i16,
    course: i16,
    height: i16,
    sats: i16,
}

impl GeoPacket {
    pub fn new(client: Vec<u8>, data: &ShortDataPacket) -> GeoPacket {
        return GeoPacket {
            imei: String::from_utf8(client).unwrap(),
            timestamp: data.timestamp,
            lat: data.lat,
            lon: data.lon,
            speed: data.speed,
            course: data.course,
            height: data.height,
            sats: data.sats,
        };
    }
}
