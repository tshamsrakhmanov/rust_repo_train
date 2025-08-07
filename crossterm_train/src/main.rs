use crossterm::{
    cursor, execute, queue,
    style::{self, Stylize, style},
    terminal::{self, size},
};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let size = terminal::size();
    let mut width: u16 = 10;
    let mut lentgh: u16 = 10;

    match size {
        Ok(result) => {
            width = result.0;
            lentgh = result.1;
        }
        Err(_) => println!("nok"),
    }

    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    for y in 0..lentgh {
        for x in 0..width {
            queue!(
                stdout,
                cursor::MoveTo(x, y),
                style::PrintStyledContent("x".red())
            )?;
            if x == 0 || y == 0 || x == width - 1 || y == lentgh - 1 {
                queue!(
                    stdout,
                    cursor::MoveTo(x, y),
                    style::PrintStyledContent("x".yellow())
                )?;
            }
        }
    }
    stdout.flush()?;
    Ok(())
    // loop {
    //     std::thread::sleep(Duration::from_millis(100));
    // }
}
