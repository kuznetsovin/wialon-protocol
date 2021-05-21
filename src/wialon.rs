use std::fmt;

#[derive(Debug)]
pub struct Packet {
    ptype: String,
    body: String
}

impl Packet {
    pub fn new(msg: &[u8]) -> Result<Packet, &str> {
        let s = String::from_utf8(msg.to_vec()).unwrap();
        if s.starts_with("#") && s.ends_with("\r\n") {

            Ok(Packet{ptype: "L".to_string(), body: "1;1".to_string()})
        } else {
            Err("Не корректное сообщение")
        }
    }
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_incorrect_msg() {
        assert_eq!(Err("Не корректное сообщение"), Packet::new(&[0x77, 0x65, 0x72, 0x0a]));
        assert_eq!(Err("Не корректное сообщение"), Packet::new(&[0x23, 0x77, 0x65, 0x72, 0x0a]));
    }

    #[test]
    fn test_packet() {        
        let p = Packet::new(&[0x23, 0x4c, 0x23, 0x31, 0x3b, 0x31, 0x0d, 0x0A]).unwrap();
        assert_eq!(p.ptype, "L");
        assert_eq!(p.body, "1;1");
    }
}
