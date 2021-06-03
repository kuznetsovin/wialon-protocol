use core::any::Any;
use std::collections::HashMap;
use std::fmt;

use crate::wialon::BodyParser;
use crate::wialon::short_data_packet::ShortDataPacket;

// trait GetParamValue<T>: fmt::Debug + fmt::Display {
//     fn get_value(&self) -> T;
// }


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Params {
    Int(i32),
    Float(f64)
    // Int(i32),
    // String(&'a str)
}

impl fmt::Display for Params {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct DataPacket {
    pub spd: ShortDataPacket,
    pub hdop: f64,
    pub inputs: i32,
    pub outputs: i32,
    pub adc: String,
    pub ibutton: String,
    pub params: HashMap<String, Params>,
}

impl From<Vec<&str>> for DataPacket {
    fn from(body: Vec<&str>) -> Self {
        let hdop = body[10].to_string().parse().unwrap();
        let inputs = body[11].to_string().parse().unwrap();
        let outputs = body[12].to_string().parse().unwrap();

        let params: Vec<&str> = body[15].split(",").collect();
        let mut params_map = HashMap::new();

        for p in params {
            let param_tuple: Vec<&str>  = p.split(":").collect();
            let param_type = param_tuple[1];
            let v: Params = match param_type {
                "1" => Params::Int(param_tuple[2].to_string().parse().unwrap()),
                "2" => Params::Float(param_tuple[2].to_string().parse().unwrap()),
                // "3" => Params::String(param_val),
                // _ => Params::String(param_tuple[2]),
                _ => Params::Int(0),
            };
            params_map.insert(param_tuple[0].to_string(), v);   
        }

        DataPacket {
            spd: ShortDataPacket::from(body[0..10].to_vec()),
            hdop,
            inputs,
            outputs,
            adc: body[13].to_string(),
            ibutton: body[14].to_string(),
            params: params_map,
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
        write!(f, "{};{};{};{};{};{}", self.spd, self.hdop, self.inputs, self.outputs,
               self.adc, self.ibutton)
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
            // && self.params == other.params
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
    
    let p = msg.params.get("test1").unwrap();
    assert_eq!(*p, Params::Int(1));
    
    let p = msg.params.get("var").unwrap();
    assert_eq!(*p, Params::Float(4.5));
}