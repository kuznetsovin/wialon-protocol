use core::any::Any;
use std::fmt;

mod short_data_packet;

use short_data_packet::{ShortDataPacket};

mod login_packet;
mod data_packet;

use login_packet::LoginPacket;
use crate::wialon::data_packet::{DataPacket, Params};

trait BodyParser: fmt::Debug + fmt::Display {
    fn as_any(&self) -> &dyn Any;
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
            "D" => Box::new(DataPacket::from(body_parts)),
            _ => return Err("Не корректное сообщение"),
        };

        return Ok(Packet { ptype: packet_type.to_string(), body: b });
    }

    pub fn get_auth_data(&self) -> Result<&LoginPacket, &str> {
        let p: &LoginPacket = match self.body.as_any().downcast_ref::<LoginPacket>() {
            Some(b) => b,
            None => return Err("Ошибка преобразования: пакет не является пакетом логина"),
        };
        Ok(p)
    }

    pub fn get_navigate_data(&self) -> Result<&ShortDataPacket, &str> {
        let p: &ShortDataPacket = match self.body.as_any().downcast_ref::<ShortDataPacket>() {
            Some(b) => b,
            None => match self.body.as_any().downcast_ref::<DataPacket>() {
                Some(d) => &d.spd,
                None => return Err("Ошибка преобразования: пакет не является с навигационными данными")
            },
        };
        Ok(p)
    }

    pub fn get_extra_param(&self, param_name: &str) -> Result<Params, &str> {
        let p: &Params = match self.body.as_any().downcast_ref::<DataPacket>() {
            Some(b) => {
                match b.params.get(param_name) {
                    Some(r) => r,
                    None => return Err("Ошибка получения параметра"),
                }
            },
            None => return Err("Ошибка преобразования: пакет не содержит доп параметров"),
        };
        Ok(*p)
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}#{}\r\n", self.ptype, self.body)
    }
}

#[test]
fn parsing_packets() {
    match Packet::from(&[0x77, 0x65, 0x72, 0x0a]) {
        Ok(_) => (),
        Err(err) => assert_eq!("Не корректное сообщение", err)
    }

    match Packet::from(&[0x23, 0x77, 0x65, 0x72, 0x0a]) {
        Ok(_) => (),
        Err(err) => assert_eq!("Не корректное сообщение", err)
    }

    match Packet::from("#L#1;1\r\n".as_bytes()) {
        Ok(p) => {
            assert_eq!(p.ptype, "L");

            let msg = p.get_auth_data().unwrap();

            assert_eq!(msg.imei, "1");
            assert_eq!(msg.password, "1");
        }
        Err(err) => panic!("{:?}", err)
    }

    use chrono::NaiveDateTime;
    match Packet::from("#SD#280421;055447;5355.09260;N;02732.40990;E;60;0;300;7\r\n".as_bytes()) {
        Ok(p) => {
            assert_eq!(p.ptype, "SD");
            let msg = p.get_navigate_data().unwrap();

            assert_eq!(msg.timestamp, NaiveDateTime::parse_from_str("280421055447", "%d%m%y%H%M%S").unwrap());
            assert_eq!(msg.lon, 53.5509260);
            assert_eq!(msg.lat, 27.3240990);
            assert_eq!(msg.speed, 60);
            assert_eq!(msg.course, 0);
            assert_eq!(msg.height, 300);
            assert_eq!(msg.sats, 7);
        }
        Err(err) => panic!("{:?}", err)
    }

    match Packet::from("#D#280421;055500;5355.09260;N;02732.40990;E;60;0;300;7;22;5;5120;;eee;test1:1:1,var:2:4.5,texttest:3:1\r\n".as_bytes()) {
        Ok(p) => {
            assert_eq!(p.ptype, "D");
            let msg = p.get_navigate_data().unwrap();

            assert_eq!(msg.timestamp, NaiveDateTime::parse_from_str("280421055500", "%d%m%y%H%M%S").unwrap());

            assert_eq!(p.get_extra_param("test1").unwrap(), Params::Int(1));
            assert_eq!(p.get_extra_param("var").unwrap(), Params::Float(4.5));
        }
        Err(err) => panic!("{:?}", err)
    }
}
