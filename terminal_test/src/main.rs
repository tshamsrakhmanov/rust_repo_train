use crossterm::style::Print;
use crossterm::{
    self, ExecutableCommand, QueueableCommand, cursor,
    event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, poll, read},
    execute,
    terminal::{
        self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};
use std::io::{self, Write, stdout};
use std::time::Duration;

fn main() -> io::Result<()> {
    // declare stdout
    let mut stdout = stdout();

    // enter alternate screen and raw mode
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    stdout.queue(cursor::MoveTo(5, 5))?;
    stdout.queue(cursor::Hide)?;
    stdout.queue(Print("s"))?;
    stdout.flush()?;

    //  LOOP START

    //     exapmle of drawing
    'main_loop: loop {
        if poll(Duration::from_millis(100))? {
            match read()? {
                Event::Key(event) => {
                    if event
                        == (KeyEvent {
                            code: KeyCode::Char('q'),
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            state: KeyEventState::NONE,
                        })
                    {
                        break 'main_loop;
                    };
                }
                _ => println!("!"),
            }
        }
    }

    //  LOOP END

    // exit alternate screen and this is the end, supposedly )))
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)
}
