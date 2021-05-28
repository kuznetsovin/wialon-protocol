use core::any::Any;
use std::fmt;
use chrono::NaiveDateTime;

trait BodyParser: fmt::Debug + fmt::Display{
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct LoginPacket {
    imei: String,
    password: String,
}

impl From<Vec<&str>> for LoginPacket {
    fn from(body: Vec<&str>) -> Self {
        LoginPacket{imei: body[0].to_string(), password: body[1].to_string()}
    }
}

impl BodyParser for LoginPacket {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Display for LoginPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{};{}", self.imei, self.password)
    }
}

impl PartialEq for LoginPacket {
    fn eq(&self, other: &Self) -> bool {
        self.imei == other.imei && self.password == other.password
    }
}

#[derive(Debug)]
pub struct ShortDataPacket {
    // date: String,
    // time: String,
    timestamp: NaiveDateTime,
    lat: f64,
    lon: f64,
    speed: i16, 
    course: i16,
    height: i16,
    sats: i16,
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


#[derive(Debug)]
pub struct Packet {
    ptype: String,
    body: Box<dyn BodyParser>,
}

impl Packet {
    pub fn from(msg: &[u8]) -> Result<Packet, &str> {
        let s = String::from_utf8(msg.to_vec()).unwrap();
        if !(s.starts_with("#") && s.ends_with("\r\n")) {
            return Err("Не корректное сообщение");
        }

        let parts: Vec<&str> = s.trim_start_matches('#').trim_end().split("#").collect(); 
        let raw_body = parts[1].to_string();
        let packet_type = parts[0]; 
        let body_parts: Vec<&str> = raw_body.split(";").collect();
        
        let b: Box<dyn BodyParser> = match packet_type {
            "L" => Box::new(LoginPacket::from(body_parts)),
            "SD" => Box::new(ShortDataPacket::from(body_parts)),
            _ => return Err("Не корректное сообщение"),
        };

        return Ok(Packet{ptype: packet_type.to_string(), body: b});    
    }

    pub fn get_auth_data(&self) -> Result<&LoginPacket, &str> {
        let p: &LoginPacket = match self.body.as_any().downcast_ref::<LoginPacket>() {
            Some(b) => b,
            None => return Err("Ошибка преобрзования: пакет не является пакетом логина"),
        };
        Ok(p)
    }

    pub fn get_navigate_data(&self) -> Result<&ShortDataPacket, &str> {
        let p: &ShortDataPacket = match self.body.as_any().downcast_ref::<ShortDataPacket>() {
            Some(b) => b,
            None => return Err("Ошибка преобрзования: пакет не является с навигационными данными логина"),
        };
        Ok(p)
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}#{}\r\n", self.ptype, self.body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_incorrect_msg() {
        match Packet::from(&[0x77, 0x65, 0x72, 0x0a]) {
            Ok(_) => (),
            Err(err) => assert_eq!("Не корректное сообщение", err)
        }

        match Packet::from(&[0x23, 0x77, 0x65, 0x72, 0x0a]) {
            Ok(_) => (),
            Err(err) => assert_eq!("Не корректное сообщение", err)
        }
    }

    #[test]
    fn test_login_packet() {
        let p = Packet::from("#L#1;1\r\n".as_bytes()).unwrap();
        assert_eq!(p.ptype, "L");
        
        let msg = p.get_auth_data().unwrap();

        assert_eq!(msg.imei, "1");
        assert_eq!(msg.password, "1");
    }

    #[test]
    fn test_short_data_packet() {    
        let test_ts = NaiveDateTime::parse_from_str("280421055220", "%d%m%y%H%M%S").unwrap();

        let p = Packet::from("#SD#280421;055220;5355.09260;N;02732.40990;E;0;0;300;7\r\n".as_bytes()).unwrap();
        assert_eq!(p.ptype, "SD");

        let msg = p.get_navigate_data().unwrap();

        assert_eq!(msg.timestamp, test_ts);
        assert_eq!(msg.lon, 53.5509260);
        assert_eq!(msg.lat, 27.3240990);
        assert_eq!(msg.speed, 0);        
        assert_eq!(msg.course, 0);        
        assert_eq!(msg.height, 300);        
        assert_eq!(msg.sats, 7);        
    }
}