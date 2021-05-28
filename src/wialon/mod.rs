use core::any::Any;
use std::fmt;

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
    date: String,
    time: String,
    lat: String,
    lat_dir: String,
    lon: String,
    lon_dir: String,
    speed: String, 
    course: String,
    height: String,
    sats: String,
}

impl From<Vec<&str>> for ShortDataPacket {
    fn from(body: Vec<&str>) -> Self {
        ShortDataPacket{
            date: body[0].to_string(),
            time: body[1].to_string(),
            lat: body[2].to_string(),
            lat_dir: body[3].to_string(),
            lon: body[4].to_string(),
            lon_dir: body[5].to_string(),
            speed: body[6].to_string(),
            course: body[7].to_string(),
            height: body[8].to_string(),
            sats: body[9].to_string(),
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
        write!(f, "{};{};{};{};{};{};{};{};{};{}", self.date, self.time, self.lat, self.lat_dir, self.lon, self.lon_dir, 
        self.speed, self.course, self.height, self.sats)
    }
}

impl PartialEq for ShortDataPacket {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date 
        && self.time == other.time
        && self.lat == other.lat
        && self.lat_dir == other.lat_dir
        && self.lon == other.lon
        && self.lon_dir == other.lon_dir
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
        let p = Packet::from(&[0x23, 0x4c, 0x23, 0x31, 0x3b, 0x31, 0x0d, 0x0A]).unwrap();
        assert_eq!(p.ptype, "L");
        
        let msg = p.get_auth_data().unwrap();

        assert_eq!(msg.imei, "1");
        assert_eq!(msg.password, "1");
    }

    #[test]
    fn test_short_data_packet() {        
        let p = Packet::from(&[0x23, 0x53, 0x44, 0x23, 0x32, 0x38, 0x30, 0x34, 0x32, 0x31, 0x3b, 0x30, 0x35, 0x35, 0x32, 
            0x32, 0x30, 0x3b, 0x35, 0x33, 0x35, 0x35, 0x2e, 0x30, 0x39, 0x32, 0x36, 0x30, 0x3b, 0x4e, 0x3b, 0x30, 0x32, 
            0x37, 0x33, 0x32, 0x2e, 0x34, 0x30, 0x39, 0x39, 0x30, 0x3b, 0x45, 0x3b, 0x30, 0x3b, 0x30, 0x3b, 0x33, 0x30, 
            0x30, 0x3b, 0x37, 0x0d, 0x0A]).unwrap();
        assert_eq!(p.ptype, "SD");

        let msg = p.get_navigate_data().unwrap();
        assert_eq!(msg.date, "280421");
        assert_eq!(msg.sats, "7");
    }
}