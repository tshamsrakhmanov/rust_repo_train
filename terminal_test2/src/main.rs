use crossterm::{
    cursor,
    event::{Event, KeyCode, poll, read},
    execute,
    style::{self, Stylize},
    terminal::{
        self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};
use std::{
    io::{Cursor, Write, stdout},
    time::Duration,
};

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();

    let screen_size = terminal::size()?;
    let dim_x = screen_size.0;
    let dim_y = screen_size.1;

    execute!(stdout, cursor::Hide)?;
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;
    'main_loop: loop {
        let mut str1 = String::new();
        for x in 0..460 {
            str1.push('a');
        }
        execute!(stdout, cursor::MoveTo(0, 0))?;
        execute!(stdout, style::PrintStyledContent(str1.magenta()))?;
        stdout.flush()?;

        if poll(Duration::from_millis(1000))? {
            if let Event::Key(event) = read()? {
                if event.code == KeyCode::Char('q') {
                    break 'main_loop;
                }
            }
        }

        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    }
    disable_raw_mode()?;
    execute!(stdout, cursor::Show)?;
    execute!(stdout, LeaveAlternateScreen)
}
