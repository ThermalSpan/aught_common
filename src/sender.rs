use command::Command;
use error::CommandResult;
use serde_json;
use std::io::Write;

pub struct Sender<'a> {
    w: &'a mut Write,
}

impl<'a> Sender<'a> {
    pub fn new(w: &'a mut Write) -> Sender {
        Sender { w }
    }

    pub fn send(
        &mut self,
        c: Command,
    ) -> CommandResult<()> {
        let message = serde_json::to_string(&c)?;
        let size = message.len() as u8;
        self.w.write(&[size])?;
        self.w.write(message.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sender() {
        let mut buffer = Vec::new();

        {
            let mut sender = Sender::new(&mut buffer);
            sender.send(Command::Clear).unwrap();
            sender.send(Command::Line(0.0, 0.0, 1.0, 2.0)).unwrap();
            sender.send(Command::Point(0.0, 0.0)).unwrap();
            sender.send(Command::Done).unwrap();
        }

        let string = String::from_utf8_lossy(&buffer);
        assert_eq!(
            string,
            "\u{7}\"Clear\"\u{1a}{\"Line\":[0.0,0.0,1.0,2.0]}\u{13}{\"Point\":[0.0,0.0]}\u{6}\"Done\""
        );
    }
}
