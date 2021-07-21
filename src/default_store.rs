use crate::store::{Store, GeoPacket};

#[derive(Copy, Clone, Debug)]
pub struct ConsoleStore {}

impl ConsoleStore{
    pub fn new() -> ConsoleStore {
        return ConsoleStore{}
    }
}

impl Store for ConsoleStore {
    fn save(&self, p: GeoPacket) {
        let packet_json = serde_json::to_string(&p).unwrap();
        println!("{:?}", packet_json);
    }
}