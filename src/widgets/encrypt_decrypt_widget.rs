use std::io::Write;

use crossterm::cursor;
use crossterm::event::KeyCode;
use crossterm::style;
use crossterm::terminal;

use crate::decrypt::decrypt;
use crate::encrypt::encrypt;
use crate::key_storage::Storage;
use crate::widgets::utils::{read_char, EncryptOrDecrypt};

#[derive(PartialEq)]
pub enum EncryptWidgetState {
    Back,
    Continue,
    End,
}

#[derive(PartialEq)]
pub struct TextResult {
    text: Option<String>,
    state: EncryptWidgetState,
}

pub fn run_choose_key_pair<W>(w: &mut W, procedure: EncryptOrDecrypt) -> crossterm::Result<()>
where
    W: Write,
{
    let mut index: usize = 0;
    let storage = Storage::load_storage();

    loop {
        crossterm::queue!(
            w,
            style::ResetColor,
            terminal::Clear(crossterm::terminal::ClearType::All),
            cursor::Hide,
            cursor::MoveTo(2, 2),
            cursor::DisableBlinking,
            if procedure == EncryptOrDecrypt::Encrypt {
                style::Print("ENCRYPT")
            } else {
                style::Print("DECRYPT")
            },
            cursor::MoveToNextLine(2),
            cursor::MoveRight(2),
            style::Print("Choose a key pair to encrypt/decrypt your message:"),
            cursor::MoveToNextLine(1),
            cursor::MoveRight(2),
        )?;

        if storage.get_key_pairs().len() == 0 {
            crossterm::queue!(
                w,
                style::Print("No key pairs found. Please create a new key pair."),
                cursor::MoveToNextLine(1),
                cursor::MoveRight(2),
            )?;
        } else {
            for id in 0..storage.get_key_pairs().len() {
                let key_pair = storage.get_key_pair_by_id(id).unwrap();
                if id == index {
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
                    style::Print(format!("{}: {}", id, key_pair.name)),
                    cursor::MoveToNextLine(1),
                    cursor::MoveRight(2)
                )?;
            }
        }

        crossterm::queue!(
            w,
            style::SetForegroundColor(style::Color::Reset),
            style::SetBackgroundColor(style::Color::Reset),
            style::SetAttribute(style::Attribute::Reset),
            cursor::MoveToNextLine(1),
            cursor::MoveRight(2),
            style::Print("Use arrow keys to navigate."),
            cursor::MoveToNextLine(1),
            cursor::MoveRight(2),
            style::Print("Press Enter button to select key or ESC to go back."),
        )?;

        w.flush()?;

        match read_char()? {
            KeyCode::Up => {
                if storage.get_key_pairs().len() == 0 {
                    continue;
                } else if index > 0 {
                    index -= 1;
                } else {
                    index = storage.get_key_pairs().len() - 1;
                }
            }
            KeyCode::Down => {
                if storage.get_key_pairs().len() == 0 {
                    continue;
                } else if index < storage.get_key_pairs().len() - 1 {
                    index += 1;
                } else {
                    index = 0;
                }
            }
            KeyCode::Enter => {
                if storage.get_key_pairs().len() == 0 {
                    continue;
                } else {
                    let entered_message = run_enter_message(w, &procedure)?;

                    match entered_message.state {
                        EncryptWidgetState::Back => {
                            continue;
                        }
                        EncryptWidgetState::Continue => {
                            let key_pair = storage.get_key_pair_by_id(index).unwrap();

                            let result = match procedure {
                                EncryptOrDecrypt::Encrypt => encrypt(
                                    &entered_message.text.unwrap(),
                                    &key_pair.modulus,
                                    &key_pair.e,
                                ),
                                EncryptOrDecrypt::Decrypt => decrypt(
                                    &entered_message.text.unwrap(),
                                    &key_pair.modulus,
                                    &key_pair.d,
                                )
                                .unwrap(),
                            };

                            let print_result = run_print_result(w, &result)?;

                            match print_result {
                                EncryptWidgetState::End => {
                                    break;
                                }
                                _ => {}
                            }
                        }
                        EncryptWidgetState::End => {
                            break;
                        }
                    }
                }
            }
            KeyCode::Esc => {
                return Ok(());
            }
            _ => {}
        };
    }
    Ok(())
}

pub fn run_enter_message<W>(
    w: &mut W,
    procedure: &EncryptOrDecrypt,
) -> crossterm::Result<TextResult>
where
    W: Write,
{
    let mut tmp_input_line: String = String::new();

    loop {
        crossterm::queue!(
            w,
            style::ResetColor,
            terminal::Clear(crossterm::terminal::ClearType::All),
            cursor::MoveTo(2, 2),
            if *procedure == EncryptOrDecrypt::Encrypt {
                style::Print("ENCRYPT")
            } else {
                style::Print("DECRYPT")
            },
            cursor::MoveToNextLine(2),
            cursor::MoveRight(2),
            style::Print(r#"Enter your message: "#),
            cursor::MoveToNextLine(1),
            cursor::MoveRight(2)
        )?;

        for line in tmp_input_line.lines() {
            crossterm::queue!(
                w,
                style::Print(line),
                cursor::MoveToNextLine(1),
                cursor::MoveRight(2),
            )?;
        }

        crossterm::queue!(
            w,
            cursor::MoveToNextLine(2),
            cursor::MoveRight(2),
            if *procedure == EncryptOrDecrypt::Encrypt {
                style::Print("Press Enter button to encrypt your message.")
            } else {
                style::Print("Press Enter button to decrypt your message.")
            },
            if tmp_input_line.len() > 0 {
                cursor::MoveTo(2, tmp_input_line.lines().count() as u16 + 5)
            } else {
                cursor::MoveTo(2, 5)
            },
        )?;

        w.flush()?;

        match read_char()? {
            KeyCode::Enter => {
                if tmp_input_line.len() == 0 {
                    continue;
                } else {
                    return Ok(TextResult {
                        text: Some(tmp_input_line.clone()),
                        state: EncryptWidgetState::Continue,
                    });
                }
            }
            KeyCode::Tab => {
                if tmp_input_line.len() == 0 {
                    continue;
                } else {
                    tmp_input_line.push('\n');
                }
            }
            KeyCode::Esc => {
                return Ok(TextResult {
                    text: None,
                    state: EncryptWidgetState::Back,
                });
            }
            KeyCode::Backspace => {
                if tmp_input_line.len() > 0 {
                    tmp_input_line.pop();
                }
            }
            KeyCode::Char(c) => {
                tmp_input_line.push(c);
            }
            _ => {}
        };
    }
}

pub fn run_print_result<W>(w: &mut W, result: &String) -> crossterm::Result<EncryptWidgetState>
where
    W: Write,
{
    crossterm::queue!(w, cursor::Hide, cursor::DisableBlinking)?;
    loop {
        crossterm::queue!(
            w,
            style::ResetColor,
            terminal::Clear(crossterm::terminal::ClearType::All),
            cursor::MoveTo(2, 2),
            style::Print("RESULT"),
            cursor::MoveToNextLine(2),
            style::Print("Your result:"),
            cursor::MoveToNextLine(1),
            style::Print(result),
            cursor::MoveToNextLine(2),
            cursor::MoveRight(2),
            style::Print("Press Enter button to go back."),
            cursor::MoveToNextLine(1),
            cursor::MoveRight(2),
            cursor::MoveToNextLine(1),
            cursor::MoveRight(2),
        )?;

        w.flush()?;

        match read_char()? {
            KeyCode::Enter => return Ok(EncryptWidgetState::End),
            KeyCode::Esc => return Ok(EncryptWidgetState::End),
            _ => {}
        };
    }
}
