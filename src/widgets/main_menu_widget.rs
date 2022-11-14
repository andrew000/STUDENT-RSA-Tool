use std::io::Write;

use crossterm::cursor;
use crossterm::event::KeyCode;
use crossterm::style;
use crossterm::terminal;

use crate::widgets::utils::{read_char, EncryptOrDecrypt};
use crate::widgets::{browse_key_storage_widget, encrypt_decrypt_widget, generate_key_widget};

const CONTROLS: &str = r#"'e' - Encrypt message
'd' - Decrypt message
'g' - Generate key pairs and save to storage
'b' - Browse key storage
'q' - quit"#;

pub fn run<W>(w: &mut W) -> crossterm::Result<()>
where
    W: Write,
{
    let mut index = 0;

    loop {
        crossterm::queue!(
            w,
            style::ResetColor,
            terminal::Clear(crossterm::terminal::ClearType::All),
            cursor::Hide,
            cursor::MoveTo(2, 2),
            cursor::DisableBlinking,
            style::Print(r#"MAIN MENU"#),
            cursor::MoveToNextLine(2),
            cursor::MoveRight(2),
            style::Print("Choose an option:"),
            cursor::MoveToNextLine(1),
            cursor::MoveRight(2),
        )?;

        for (i, line) in CONTROLS.split('\n').enumerate() {
            if i == index {
                crossterm::queue!(
                    w,
                    style::SetForegroundColor(style::Color::Black),
                    style::SetBackgroundColor(style::Color::White),
                    style::SetAttribute(style::Attribute::Bold),
                )?;
            } else {
                crossterm::queue!(
                    w,
                    style::SetForegroundColor(style::Color::Reset),
                    style::SetBackgroundColor(style::Color::Reset),
                    style::SetAttribute(style::Attribute::Reset),
                )?;
            }
            crossterm::queue!(
                w,
                style::Print(line),
                cursor::MoveToNextLine(1),
                cursor::MoveRight(2),
            )?;
        }

        crossterm::queue!(
            w,
            style::SetForegroundColor(style::Color::Reset),
            style::SetBackgroundColor(style::Color::Reset),
            style::SetAttribute(style::Attribute::Reset),
            cursor::MoveToNextLine(1),
            cursor::MoveRight(2),
            style::Print(r#"Use arrow keys to navigate."#),
            cursor::MoveToNextLine(1),
            cursor::MoveRight(2),
            style::Print(r#"Press Enter to select."#),
            cursor::MoveToNextLine(1),
            cursor::MoveRight(2),
            style::Print(r#"Press 'q' to quit."#),
        )?;

        w.flush()?;

        match read_char()? {
            KeyCode::Char('e') => {
                encrypt_decrypt_widget::run_choose_key_pair(w, EncryptOrDecrypt::Encrypt)?
            }
            KeyCode::Char('d') => {
                encrypt_decrypt_widget::run_choose_key_pair(w, EncryptOrDecrypt::Decrypt)?
            }
            KeyCode::Char('g') => generate_key_widget::run(w)?,
            KeyCode::Char('b') => browse_key_storage_widget::run(w)?,
            KeyCode::Up => {
                if index > 0 {
                    index -= 1;
                } else {
                    index = CONTROLS.split('\n').count() - 1;
                }
            }
            KeyCode::Down => {
                if index < CONTROLS.split('\n').count() - 1 {
                    index += 1;
                } else {
                    index = 0;
                }
            }
            KeyCode::Enter => match index {
                0 => encrypt_decrypt_widget::run_choose_key_pair(w, EncryptOrDecrypt::Encrypt)?,
                1 => encrypt_decrypt_widget::run_choose_key_pair(w, EncryptOrDecrypt::Decrypt)?,
                2 => generate_key_widget::run(w)?,
                3 => browse_key_storage_widget::run(w)?,
                4 => break,
                _ => {}
            },
            KeyCode::Esc | KeyCode::Char('q') => break,
            _ => {}
        };
    }
    Ok(())
}
