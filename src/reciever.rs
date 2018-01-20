use command::Command;
use serde_json;
use std::io::{Bytes, Read};

pub struct Reciever<R: Read> {
    b: Bytes<R>,
}

impl<R: Read> Reciever<R> {
    pub fn new(r: R) -> Reciever<R> {
        Reciever { b: r.bytes() }
    }
}

impl<R: Read> Iterator for Reciever<R> {
    type Item = Command;

    fn next(&mut self) -> Option<Command> {
        let message_size = match self.b.next() {
            Some(Ok(b)) => b,
            _ => return None,
        };

        let mut buffer = Vec::new();
        for _ in 0..message_size {
            let b = match self.b.next() {
                Some(Ok(b)) => b,
                _ => return None,
            };
            buffer.push(b);
        }

        let json = String::from_utf8_lossy(&buffer);
        serde_json::from_str(&json).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sender::Sender;

    #[test]
    fn sender_combo() {
        let mut buffer = Vec::new();

        {
            let mut sender = Sender::new(&mut buffer);
            sender.send(Command::Clear).unwrap();
            sender.send(Command::Line(0.0, 0.0, 1.0, 2.0)).unwrap();
            sender.send(Command::Point(0.0, 0.0)).unwrap();
            sender.send(Command::Done).unwrap();
        }

        {
            let a: &[u8] = &buffer;
            let mut r = Reciever::new(a);

            let c1 = r.next().unwrap();
            let c2 = r.next().unwrap();
            let c3 = r.next().unwrap();
            let c4 = r.next().unwrap();

            assert_eq!(r.next().is_none(), true);

            let json1 = serde_json::to_string(&c1).unwrap();
            assert_eq!(json1, "\"Clear\"");

            let json2 = serde_json::to_string(&c2).unwrap();
            assert_eq!(json2, "{\"Line\":[0.0,0.0,1.0,2.0]}");

            let json3 = serde_json::to_string(&c3).unwrap();
            assert_eq!(json3, "{\"Point\":[0.0,0.0]}");

            let json4 = serde_json::to_string(&c4).unwrap();
            assert_eq!(json4, "\"Done\"");
        }
    }
}
