use std::fmt;

#[derive(Debug)]
struct LoginPacket {
    imei: String,
    password: String,
}

impl From<Vec<&str>> for LoginPacket {
    fn from(body: Vec<&str>) -> Self {
        LoginPacket{imei: body[0].to_string(), password: body[1].to_string()}
    }
}

impl fmt::Display for LoginPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.imei, self.password)
    }
}

impl PartialEq for LoginPacket {
    fn eq(&self, other: &Self) -> bool {
        self.imei == other.imei && self.password == other.password
    }
}


#[derive(Debug)]
pub struct Packet {
    ptype: String,
    body: LoginPacket,
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.ptype == other.ptype && self.body == other.body
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}#{}\r\n", self.ptype, self.body)
    }
}

pub fn parse_packet(msg: &[u8]) -> Result<Packet, &str> {
    let s = String::from_utf8(msg.to_vec()).unwrap();
    if s.starts_with("#") && s.ends_with("\r\n") {
        let parts: Vec<&str> = s.trim_start_matches('#').trim_end().split("#").collect(); 
        let raw_body = parts[1].to_string();
        let body_parts: Vec<&str> = raw_body.split(";").collect();

        Ok(Packet{ptype: parts[0].to_string(), body: LoginPacket::from(body_parts)})
    } else {
        Err("Не корректное сообщение")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_incorrect_msg() {
        assert_eq!(Err("Не корректное сообщение"), parse_packet(&[0x77, 0x65, 0x72, 0x0a]));
        assert_eq!(Err("Не корректное сообщение"), parse_packet(&[0x23, 0x77, 0x65, 0x72, 0x0a]));
    }

    #[test]
    fn test_packet() {        
        let p = parse_packet(&[0x23, 0x4c, 0x23, 0x31, 0x3b, 0x31, 0x0d, 0x0A]).unwrap();
        assert_eq!(p.ptype, "L");
        assert_eq!(p.body.imei, "1");
        assert_eq!(p.body.password, "1");
    }
}
