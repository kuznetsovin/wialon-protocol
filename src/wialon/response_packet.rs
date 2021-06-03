use std::fmt;

#[derive(Debug)]
pub struct ResponsePacket {
    pub ptype: String,    
    pub code: i8,
}

impl fmt::Display for ResponsePacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}#{}\r\n", self.ptype, self.code)
    }
}
