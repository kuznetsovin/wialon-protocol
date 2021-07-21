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

pub trait Store {
    fn save(&self, p: GeoPacket);
}

#[derive(Copy, Clone, Debug)]
pub struct ConsoleStore {}

impl ConsoleStore{
    pub fn new() -> ConsoleStore {
        return ConsoleStore{}
    }
}

impl Store for ConsoleStore {
    fn save(&self, p: GeoPacket) {
        let packet_json = serde_json::to_string(&p).unwrap();
        println!("{:?}", packet_json);
    }
}