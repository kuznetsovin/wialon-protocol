use core::any::Any;

use chrono::NaiveDateTime;
use std::fmt;

use crate::wialon::BodyParser;

#[derive(Debug)]
pub struct ShortDataPacket {
    pub timestamp: NaiveDateTime,
    pub lat: f64,
    pub lon: f64,
    pub speed: i16, 
    pub course: i16,
    pub height: i16,
    pub sats: i16,
}

impl From<Vec<&str>> for ShortDataPacket {
    fn from(body: Vec<&str>) -> Self {
        let mut ts: String = body[0].to_string();
        ts.push_str(body[1]);

        let mut lon: f64 = body[2].to_string().parse().unwrap();
        lon = lon / 100.0;
        if body[3] != "N" {
            lon = lon * -1.0
        }

        let mut lat: f64 = body[4].to_string().parse().unwrap();
        lat = lat / 100.0;
        if body[5] != "E" {
            lon = lon * -1.0
        }

        ShortDataPacket{
            timestamp:  NaiveDateTime::parse_from_str(ts.as_str(), "%d%m%y%H%M%S").unwrap(),
            lat: lat,
            lon: lon,
            speed: body[6].to_string().parse().unwrap(),
            course: body[7].to_string().parse().unwrap(),
            height: body[8].to_string().parse().unwrap(),
            sats: body[9].to_string().parse().unwrap(),
        }
    }
}

impl BodyParser for ShortDataPacket {
    fn as_any(&self) -> &dyn Any {
        self
    }
}


impl fmt::Display for ShortDataPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{};{};{};{};{};{};{}", self.timestamp, self.lat, self.lon, 
        self.speed, self.course, self.height, self.sats)
    }
}

impl PartialEq for ShortDataPacket {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp 
        && self.lat == other.lat
        && self.lon == other.lon
        && self.speed == other.speed
        && self.course == other.course
        && self.height == other.height
        && self.sats == other.sats
    }
}

#[test]
fn test_short_data_packet_body() {    
    let test_data = vec!("280421", "055220", "5355.09260", "N", "02732.40990", "E", "0", "0", "300", "7");
    let msg = ShortDataPacket::from(test_data);
    
    let test_ts = NaiveDateTime::parse_from_str("280421055220", "%d%m%y%H%M%S").unwrap();
    assert_eq!(msg.timestamp, test_ts);
    assert_eq!(msg.lon, 53.5509260);
    assert_eq!(msg.lat, 27.3240990);
    assert_eq!(msg.speed, 0);        
    assert_eq!(msg.course, 0);        
    assert_eq!(msg.height, 300);        
    assert_eq!(msg.sats, 7);        

    let test_data = vec!("280421", "055447", "5355.09260", "N", "02732.40990", "E", "60", "0", "300", "7");
    let msg = ShortDataPacket::from(test_data);

    let test_ts = NaiveDateTime::parse_from_str("280421055447", "%d%m%y%H%M%S").unwrap();
    assert_eq!(msg.timestamp, test_ts);
    assert_eq!(msg.speed, 60);        
}