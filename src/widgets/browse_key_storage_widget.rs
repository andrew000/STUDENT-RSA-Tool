use std::io::Write;

use crossterm::cursor;
use crossterm::event::KeyCode;
use crossterm::style;
use crossterm::terminal;

use crate::key_storage::Storage;
use crate::widgets::utils::read_char;

pub fn run<W>(w: &mut W) -> crossterm::Result<()>
where
    W: Write,
{
    let mut index: usize = 0;
    let mut storage = Storage::load_storage();

    loop {
        crossterm::queue!(
            w,
            style::ResetColor,
            terminal::Clear(crossterm::terminal::ClearType::All),
            cursor::Hide,
            cursor::MoveTo(2, 2),
            cursor::DisableBlinking,
            style::Print("KEY BROWSER"),
            cursor::MoveToNextLine(2),
            cursor::MoveRight(2),
        )?;

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
            style::Print("Press DELETE button to delete selected key or ESC to go back."),
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
            KeyCode::Delete => {
                storage.delete_key_pair_by_id(index);
                storage.save_storage();
                index = 0;
            }
            KeyCode::Esc => {
                return Ok(());
            }
            _ => {}
        };
    }
}
