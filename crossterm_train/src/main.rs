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
    // execute!(strout, term)

    for y in 0..lentgh {
        for x in 0..width {
            // if (y == 0 || y == a - 1) || (x == 0 || x == b - 1) {
            //     // in this loop we are more efficient by not flushing the buffer.
            //     queue!(
            //         stdout,
            //         cursor::MoveTo(x, y),
            //         style::PrintStyledContent("█".red())
            //     )?;
            // }
            if x > 2 && x < width - 2 && y > 2 && y < lentgh - 2 {
                queue!(
                    stdout,
                    cursor::MoveTo(x, y),
                    style::PrintStyledContent("X".red())
                )?;
            }
        }
    }
    stdout.flush()?;
    Ok(())
}
