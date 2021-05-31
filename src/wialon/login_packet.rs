use core::any::Any;

use std::fmt;

use crate::wialon::BodyParser;

#[derive(Debug)]
pub struct LoginPacket {
    pub imei: String,
    pub password: String,
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

#[test]
fn test_login_packet_body() {
    let test_data = vec!("1", "1");
    let msg = LoginPacket::from(test_data);

    assert_eq!(msg.imei, "1");
    assert_eq!(msg.password, "1");
}
