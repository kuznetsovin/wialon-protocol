pub fn parse_packet(b: &[u8]) -> Result<String, &str> {
    if b[0] == 0x23 && b[b.len()-2..] == [0x0D, 0x0A] {
        Ok(String::from_utf8(b.to_vec()).unwrap())
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
    fn parsing_correct_msg() {
        assert_eq!(Ok(String::from("#L#1;1\r\n")), parse_packet(&[0x23, 0x4c, 0x23, 0x31, 0x3b, 0x31, 0x0d, 0x0A]));
    }
}
