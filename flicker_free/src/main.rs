use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEvent, poll, read},
    execute,
    style::Print,
    style::{self, Stylize},
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode, size,
    },
};
use std::time::Duration;
use std::{
    collections::HashMap,
    io::{self, Write},
};
fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    let screen_size = size()?;
    let dim_x = screen_size.0;
    let dim_y = screen_size.1;

    let mut prev_screen_buffer: HashMap<(u16, u16), Color> = HashMap::new();
    prev_screen_buffer = scr_0(dim_x, dim_y);

    let mut flag = false;

    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, cursor::Hide)?;

    'main_loop: loop {
        if poll(Duration::from_millis(30))? {
            if let Event::Key(event) = read()? {
                if event.code == KeyCode::Char('q') {
                    break 'main_loop;
                }
            }
        }

        // SECTION 1 - UPDATE GLOBAL BUFFER
        let mut new_screen_buffer: HashMap<(u16, u16), Color> = HashMap::new();
        flag = !flag;
        if flag {
            new_screen_buffer = scr_0(dim_x, dim_y);
        } else {
            new_screen_buffer = scr_1(dim_x, dim_y);
        }

        // SECTION 2 - UPDATE DRAWING BUFFER, CONVERT TO STRING AND DRAW
        let mut screen_buffer_text = String::new();

        for y in 0..dim_y {
            for x in 0..dim_x {
                let temp_pos = (x, y);
                let old_unwrap = prev_screen_buffer.get_key_value(&temp_pos);
                let new_unwrap = new_screen_buffer.get_key_value(&temp_pos);
                let mut is_in_old = false;
                let mut is_in_new = false;
                match old_unwrap {
                    None => {}
                    Some(_) => {
                        is_in_old = true;
                    }
                }
                match new_unwrap {
                    None => {}
                    Some(_) => {
                        is_in_new = true;
                    }
                }

                if is_in_old && is_in_new {
                    //
                } else if is_in_old && !is_in_new {
                    execute!(stdout, cursor::MoveTo(x, y))?;
                    execute!(stdout, style::PrintStyledContent(" ".magenta()))?;
                    //
                } else if !is_in_old && is_in_new {
                    //
                } else if !is_in_old && !is_in_new {
                    //
                }
            }
        }

        execute!(stdout, cursor::MoveTo(0, 0))?;
        execute!(stdout, Print(screen_buffer_text))?;

        stdout.flush()?;
    }

    disable_raw_mode()?;
    execute!(stdout, cursor::Show)?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

enum Color {
    WHITE,
    BLUE,
    RED,
}

fn scr_0(dim_x: u16, dim_y: u16) -> HashMap<(u16, u16), Color> {
    let mut screen_buffer: HashMap<(u16, u16), Color> = HashMap::new();

    for y in 0..dim_y {
        for x in 0..dim_x {
            if x > 10 && x < dim_x - 10 {
                screen_buffer.insert((x, y), Color::RED);
            }
        }
    }

    screen_buffer
}

fn scr_1(dim_x: u16, dim_y: u16) -> HashMap<(u16, u16), Color> {
    let mut screen_buffer: HashMap<(u16, u16), Color> = HashMap::new();

    for y in 0..dim_y {
        for x in 0..dim_x {
            if x > 15 && x < dim_x - 15 {
                screen_buffer.insert((x, y), Color::BLUE);
            }
        }
    }

    screen_buffer
}
