use core::any::Any;

use std::fmt;

use crate::wialon::BodyParser;
use crate::wialon::short_data_packet::ShortDataPacket;

#[derive(Debug)]
pub struct DataPacket {
    pub spd: ShortDataPacket,
    pub hdop: f64,
    pub inputs: i32,
    pub outputs: i32,
    pub adc: String,
    pub ibutton: String,
    pub params: String,
}

impl From<Vec<&str>> for DataPacket {
    fn from(body: Vec<&str>) -> Self {
        let hdop = body[10].to_string().parse().unwrap();
        let inputs = body[11].to_string().parse().unwrap();
        let outputs = body[12].to_string().parse().unwrap();

        DataPacket {
            spd: ShortDataPacket::from(body[0..10].to_vec()),
            hdop,
            inputs,
            outputs,
            adc: body[13].to_string(),
            ibutton: body[14].to_string(),
            params: body[15].to_string(),
        }
    }
}

impl BodyParser for DataPacket {
    fn as_any(&self) -> &dyn Any {
        self
    }
}


impl fmt::Display for DataPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{};{};{};{};{};{};{};", self.spd, self.hdop, self.inputs, self.outputs,
               self.adc, self.ibutton, self.params)
    }
}

impl PartialEq for DataPacket {
    fn eq(&self, other: &Self) -> bool {
        self.spd == other.spd
            && self.hdop == other.hdop
            && self.inputs == other.inputs
            && self.outputs == other.outputs
            && self.adc == other.adc
            && self.ibutton == other.ibutton
            && self.params == other.params
    }
}

#[test]
fn test_short_data_packet_body() {
    use chrono::NaiveDateTime;
    
    let test_data = vec!("280421", "055429", "5355.09260", "N", "02732.40990",
                         "E", "0", "0", "300", "7", "22", "5", "0", "", "NA", "test1:1:1,var:2:4.5,texttest:3:1");
    let msg = DataPacket::from(test_data);

    let test_ts = NaiveDateTime::parse_from_str("280421055429", "%d%m%y%H%M%S").unwrap();
    assert_eq!(msg.spd.timestamp, test_ts);
    assert_eq!(msg.spd.lon, 53.5509260);
    assert_eq!(msg.spd.lat, 27.3240990);
    assert_eq!(msg.spd.speed, 0);
    assert_eq!(msg.spd.course, 0);
    assert_eq!(msg.spd.height, 300);
    assert_eq!(msg.spd.sats, 7);
    assert_eq!(msg.hdop, 22.0);
    assert_eq!(msg.adc, "");
    assert_eq!(msg.params, "test1:1:1,var:2:4.5,texttest:3:1");
}