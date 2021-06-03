use std::fmt;
use std::str;

mod short_data_packet;

use short_data_packet::ShortDataPacket;

mod data_packet;
use data_packet::{DataPacket, Params};

mod login_packet;
use login_packet::LoginPacket;

mod response_packet;
use response_packet::ResponsePacket;

#[derive(Debug)]
enum PacketTypes<'a> {
    LoginPacket(LoginPacket),
    ShortDataPacket(ShortDataPacket),
    DataPacket(DataPacket<'a>),
}
impl fmt::Display for PacketTypes<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Packet<'a> {
    ptype: String,
    body: PacketTypes<'a>,
}

impl<'a> Packet<'a> {
    pub fn response(&self, result_code: i8) -> Result<ResponsePacket, &str> {
        let ptype: String = match self.ptype.as_str() {
            "L" => String::from("AL"),
            "SD" => String::from("ASD"),
            "D" => String::from("AD"),
            _ => return Err("Неизвестный тип пакета")
        };

        Ok(ResponsePacket{
            ptype,
            code: result_code
        })
    }
    pub fn from(msg: &'a [u8]) -> Result<Packet, &str> {
        let s = str::from_utf8(msg).unwrap();
        if !(s.starts_with("#") && s.ends_with("\r\n")) {
            return Err("Не корректное сообщение");
        }

        let t = s;
        let parts: Vec<&'a str> = t.trim_start_matches('#').trim_end().split("#").collect();
        let packet_type = parts[0];
        let body_parts: Vec<&str> = parts[1].split(";").collect();

        let b: PacketTypes = match packet_type {
            "L" => PacketTypes::LoginPacket(LoginPacket::from(body_parts)),
            "SD" => PacketTypes::ShortDataPacket(ShortDataPacket::from(body_parts)),
            "D" => PacketTypes::DataPacket(DataPacket::from(body_parts)),
            _ => return Err("Не корректное сообщение"),
        };

        return Ok(Packet {
            ptype: packet_type.to_string(),
            body: b,
        });
    }

    pub fn get_auth_data(&self) -> Result<&LoginPacket, &str> {
        let p: &LoginPacket = match &self.body {
            PacketTypes::LoginPacket(b) => b,
            PacketTypes::ShortDataPacket(_) => return Err("Не верный тип пакета"),
            PacketTypes::DataPacket(_) => return Err("Не верный тип пакета"),
        };
        Ok(p)
    }

    pub fn get_navigate_data(&self) -> Result<&ShortDataPacket, &str> {
        let p: &ShortDataPacket = match &self.body {
            PacketTypes::LoginPacket(_) => return Err("Не верный тип пакета"),
            PacketTypes::ShortDataPacket(b) => b,
            PacketTypes::DataPacket(b) => &b.spd,
        };
        Ok(p)
    }

    pub fn get_extra_param(&self, param_name: &str) -> Result<&Params, &str> {
        let p: &DataPacket<'_> = match &self.body {
            PacketTypes::LoginPacket(_) => return Err("Пакет не содержит экстра данных"),
            PacketTypes::ShortDataPacket(_) => return Err("Пакет не содержит экстра данных"),
            PacketTypes::DataPacket(b) => b,
        };

        let r: &Params = match p.params.get(param_name) {
            Some(r) => r,
            None => return Err("Ошибка получения параметра"),
        };
        Ok(r)
    }
}

impl fmt::Display for Packet<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}#{}\r\n", self.ptype, self.body)
    }
}

#[test]
fn parsing_packets() {
    match Packet::from(&[0x77, 0x65, 0x72, 0x0a]) {
        Ok(_) => (),
        Err(err) => assert_eq!("Не корректное сообщение", err),
    }

    match Packet::from(&[0x23, 0x77, 0x65, 0x72, 0x0a]) {
        Ok(_) => (),
        Err(err) => assert_eq!("Не корректное сообщение", err),
    }

    match Packet::from("#L#1;1\r\n".as_bytes()) {
        Ok(p) => {
            assert_eq!(p.ptype, "L");

            let msg = p.get_auth_data().unwrap();

            assert_eq!(msg.imei, "1");
            assert_eq!(msg.password, "1");
        }
        Err(err) => panic!("{:?}", err),
    }

    use chrono::NaiveDateTime;
    match Packet::from("#SD#280421;055447;5355.09260;N;02732.40990;E;60;0;300;7\r\n".as_bytes()) {
        Ok(p) => {
            assert_eq!(p.ptype, "SD");
            let msg = p.get_navigate_data().unwrap();

            assert_eq!(
                msg.timestamp,
                NaiveDateTime::parse_from_str("280421055447", "%d%m%y%H%M%S").unwrap()
            );
            assert_eq!(msg.lon, 53.5509260);
            assert_eq!(msg.lat, 27.3240990);
            assert_eq!(msg.speed, 60);
            assert_eq!(msg.course, 0);
            assert_eq!(msg.height, 300);
            assert_eq!(msg.sats, 7);
        }
        Err(err) => panic!("{:?}", err),
    }

    match Packet::from("#D#280421;055500;5355.09260;N;02732.40990;E;60;0;300;7;22;5;5120;;eee;test1:1:1,var:2:4.5,texttest:3:1\r\n".as_bytes()) {
        Ok(p) => {
            assert_eq!(p.ptype, "D");
            let msg = p.get_navigate_data().unwrap();

            assert_eq!(msg.timestamp, NaiveDateTime::parse_from_str("280421055500", "%d%m%y%H%M%S").unwrap());

            assert_eq!(p.get_extra_param("test1").unwrap(), &Params::Int(1));
            assert_eq!(p.get_extra_param("var").unwrap(), &Params::Float(4.5));
        }
        Err(err) => panic!("{:?}", err)
    }
}

#[test]
fn response_packets() {
    match Packet::from("#L#1;1\r\n".as_bytes()) {
        Ok(p) => assert_eq!(p.response(1).unwrap().to_string(), "#AL#1\r\n"),
        Err(err) => panic!("{:?}", err),
    }

    match Packet::from("#SD#280421;055447;5355.09260;N;02732.40990;E;60;0;300;7\r\n".as_bytes()) {
        Ok(p) => {
            let r = p.response(1).unwrap();
            assert_eq!(r.to_string(), "#ASD#1\r\n")
        }
        Err(err) => panic!("{:?}", err),
    }

    match Packet::from("#D#280421;055500;5355.09260;N;02732.40990;E;60;0;300;7;22;5;5120;;eee;test1:1:1,var:2:4.5,texttest:3:1\r\n".as_bytes()) {
        Ok(p) => {
            let r = p.response(1).unwrap();
            assert_eq!(r.to_string(), "#AD#1\r\n")
        }
        Err(err) => panic!("{:?}", err)
    }
}