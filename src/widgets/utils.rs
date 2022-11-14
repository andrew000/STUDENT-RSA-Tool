use crossterm::event::{read, Event, KeyCode};

#[derive(PartialEq)]
pub enum EncryptOrDecrypt {
    Encrypt,
    Decrypt,
}

pub fn read_char() -> crossterm::Result<KeyCode> {
    loop {
        if let Event::Key(key_event) = read()? {
            return Ok(key_event.code);
        }
    }
}
