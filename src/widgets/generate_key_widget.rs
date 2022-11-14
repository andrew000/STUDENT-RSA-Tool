use std::io::Write;
use std::time::Duration;

use crossterm::cursor;
use crossterm::event::{Event, KeyCode};
use crossterm::style;
use crossterm::terminal;

use crate::key_generator;
use crate::key_storage::{KeyPair, Storage};
use crate::widgets::utils::read_char;

pub fn run<W>(w: &mut W) -> crossterm::Result<()>
where
    W: Write,
{
    let mut tmp_input_line: String = String::new();
    let mut key_size: i64 = -1;

    loop {
        crossterm::queue!(
            w,
            style::ResetColor,
            terminal::Clear(crossterm::terminal::ClearType::All),
            cursor::MoveTo(2, 2),
            cursor::EnableBlinking,
            cursor::Show,
        )?;

        crossterm::queue!(
            w,
            style::Print("KEY GENERATOR"),
            cursor::MoveToNextLine(2),
            cursor::MoveRight(2),
        )?;

        if key_size == -1 {
            crossterm::queue!(
                w,
                style::Print(format!(r#"Enter key size (1024): {}"#, &tmp_input_line)),
                cursor::MoveToNextLine(2),
                cursor::MoveRight(2),
                style::Print("Press Enter to generate key or ESC to cancel."),
            )?;
        }
        crossterm::queue!(
            w,
            cursor::MoveTo(
                format!(r#"Enter key size (1024): {}"#, &tmp_input_line).len() as u16 + 2,
                4,
            )
        )?;

        w.flush()?;

        if let Event::Key(key_event) = crossterm::event::read()? {
            match key_event.code {
                KeyCode::Enter => {
                    if tmp_input_line.is_empty() {
                        key_size = 1024;
                        break;
                    } else if let Ok(size) = tmp_input_line.parse::<usize>() {
                        if size >= 1024 {
                            key_size = tmp_input_line.parse().unwrap();
                            tmp_input_line.clear();
                            break;
                        } else {
                            tmp_input_line = String::new();
                            continue;
                        }
                    } else {
                        tmp_input_line.clear();
                        continue;
                    }
                }
                KeyCode::Backspace => {
                    tmp_input_line.pop();
                }
                KeyCode::Esc => {
                    return Ok(());
                }
                KeyCode::Char(c) => {
                    tmp_input_line.push(c);
                }
                _ => {}
            }
        }
    }

    crossterm::queue!(
        w,
        style::ResetColor,
        terminal::Clear(crossterm::terminal::ClearType::All),
        cursor::Hide,
        cursor::MoveTo(2, 2),
        style::Print(format!(r#"Selected key size: {}"#, key_size)),
        cursor::MoveToNextLine(2),
        cursor::MoveRight(2),
        style::Print("Generating key pair"),
    )?;

    let thread_join_handle =
        std::thread::spawn(move || key_generator::generate_key_pair(key_size as usize));

    while !thread_join_handle.is_finished() {
        crossterm::queue!(w, crossterm::style::Print("."),)?;

        w.flush()?;

        std::thread::sleep(Duration::from_millis(200));
    }

    let key_pair: KeyPair = thread_join_handle.join().unwrap();

    loop {
        crossterm::queue!(
            w,
            terminal::Clear(crossterm::terminal::ClearType::All),
            cursor::Hide,
            cursor::MoveTo(2, 2),
            style::Print(format!(r#"Generated key pair with size: {}"#, key_size)),
            cursor::MoveToNextLine(2),
            cursor::MoveRight(2),
            style::Print(format!(
                r#"p: {}..."#,
                key_pair.p.to_string()[..key_pair.p.to_string().len() / 8].to_string()
            )),
            cursor::MoveToNextLine(1),
            cursor::MoveRight(2),
            style::Print(format!(
                r#"q: {}..."#,
                key_pair.q.to_string()[..key_pair.q.to_string().len() / 8].to_string()
            )),
            cursor::MoveToNextLine(1),
            cursor::MoveRight(2),
            style::Print(format!(
                r#"n: {}..."#,
                key_pair.modulus.to_string()[..key_pair.modulus.to_string().len() / 8].to_string()
            )),
            cursor::MoveToNextLine(1),
            cursor::MoveRight(2),
            style::Print(format!(r#"e: {}"#, key_pair.e.to_string())),
            cursor::MoveToNextLine(1),
            cursor::MoveRight(2),
            style::Print(format!(
                r#"d: {}..."#,
                key_pair.d.to_string()[..key_pair.d.to_string().len() / 8].to_string()
            )),
            cursor::MoveToNextLine(2),
            cursor::MoveRight(2),
            style::Print("Save key pair? (y/n)"),
        )?;

        w.flush()?;

        match read_char()? {
            KeyCode::Char('y') => {
                run_choose_key_pair_name(w, key_pair)?;
                break;
            }
            KeyCode::Char('n') => {
                break;
            }
            _ => {}
        }
    }

    Ok(())
}

fn run_choose_key_pair_name<W>(w: &mut W, mut key_pair: KeyPair) -> crossterm::Result<()>
where
    W: Write,
{
    let mut tmp_input_line: String = String::new();

    loop {
        crossterm::queue!(
            w,
            terminal::Clear(crossterm::terminal::ClearType::All),
            cursor::Hide,
            cursor::MoveTo(2, 2),
            style::Print(format!(r#"Enter key name (default): {}"#, tmp_input_line)),
        )?;

        w.flush()?;

        if let Event::Key(key_event) = crossterm::event::read()? {
            match key_event.code {
                KeyCode::Enter => {
                    key_pair.name = tmp_input_line.clone();
                    let mut storage = Storage::load_storage();
                    storage.add_key_pair(key_pair);
                    storage.save_storage();
                    break;
                }
                KeyCode::Backspace => {
                    tmp_input_line.pop();
                }
                KeyCode::Char(c) => {
                    tmp_input_line.push(c);
                }
                _ => {}
            }
        }
    }

    Ok(())
}
