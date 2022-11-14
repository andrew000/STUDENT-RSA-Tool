use std::io;

use crossterm::cursor;
use crossterm::style;
use crossterm::terminal;

mod decrypt;
mod encrypt;
mod key_generator;
mod key_storage;
mod utils;
mod widgets;

fn run<W>(w: &mut W) -> crossterm::Result<()>
    where
        W: io::Write,
{
    crossterm::execute!(w, terminal::EnterAlternateScreen,)?;

    terminal::enable_raw_mode()?;
    widgets::main_menu_widget::run(w)?;

    crossterm::execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()
}

fn main() -> crossterm::Result<()> {
    let mut stdout = io::stdout();
    run(&mut stdout)
}
