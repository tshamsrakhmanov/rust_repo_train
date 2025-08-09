use crossterm::{
    cursor, execute, queue,
    style::{self, Stylize, style},
    terminal,
};
use std::{
    io::{self, Write},
    time::Duration,
};

use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;

fn main() {
    loop {
        for x in 1..70 {
            draw(x, 0);
            std::thread::sleep(Duration::from_millis(25));
        }
    }
}

fn draw(x: u16, y: u16) -> io::Result<()> {
    match enable_raw_mode() {
        Ok(_) => {
            //..
        }
        Err(_) => {
            panic!("error");
        }
    }

    let mut stdout = io::stdout();

    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    queue!(
        stdout,
        cursor::MoveTo(0, 0),
        style::PrintStyledContent("█".magenta())
    )?;

    queue!(
        stdout,
        cursor::MoveTo(x, y),
        style::PrintStyledContent("X".magenta())
    )?;

    queue!(stdout, cursor::Hide)?;

    match disable_raw_mode() {
        Ok(_) => {
            //..
        }
        Err(_) => {
            panic!("error");
        }
    }

    stdout.flush()?;
    Ok(())
}
