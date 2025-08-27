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
use std::{
    collections::HashMap,
    io::{self, Write},
};
use std::{collections::HashSet, time::Duration};
fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    let screen_size = size()?;
    let dim_x = screen_size.0;
    let dim_y = screen_size.1;

    let mut prev_screen_buffer: HashMap<(u16, u16), Color> = gen_section(dim_x, dim_y);
    let mut next_screen_buffer: HashMap<(u16, u16), Color> = HashMap::new();

    let mut flag = true;

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

        // SECTION 1 - UPDATE NEXT BUFFER
        flag = !flag;

        if flag {
            next_screen_buffer = gen_full(dim_x, dim_y);
        } else {
            next_screen_buffer = gen_section(dim_x, dim_y);
        }

        // SECTION 2 - UPDATE DRAWING BUFFER, CONVERT TO STRING AND DRAW

        let keys_prev_buffer: HashSet<&(u16, u16)> = prev_screen_buffer.keys().collect();
        let keys_next_buffer: HashSet<&(u16, u16)> = next_screen_buffer.keys().collect();

        let to_delete = keys_prev_buffer.difference(&keys_next_buffer);
        let to_redraw = keys_prev_buffer.intersection(&keys_next_buffer);
        let to_draw_new = keys_next_buffer.difference(&keys_prev_buffer);

        for pos in to_delete {
            execute!(stdout, cursor::MoveTo(pos.0, pos.1))?;
            execute!(stdout, Print(" "))?;
        }
        for pos in to_redraw {
            execute!(stdout, cursor::MoveTo(pos.0, pos.1))?;
            execute!(stdout, Print("O"))?;
        }
        for pos in to_draw_new {
            execute!(stdout, cursor::MoveTo(pos.0, pos.1))?;
            execute!(stdout, Print("O"))?;
        }

        prev_screen_buffer = next_screen_buffer;

        // execute!(stdout, Clear(ClearType::All))?;
        //
        // for y in 0..dim_y {
        //     for x in 0..dim_x {
        //         let c = next_screen_buffer.get_key_value(&(x, y));
        //         match c {
        //             None => {}
        //             Some(_) => {
        //                 execute!(stdout, cursor::MoveTo(x, y))?;
        //                 execute!(stdout, Print("O"))?;
        //             }
        //         }
        //     }
        // }

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

fn gen_section(dim_x: u16, dim_y: u16) -> HashMap<(u16, u16), Color> {
    let mut screen_buffer: HashMap<(u16, u16), Color> = HashMap::new();
    for y in 0..dim_y {
        for x in 0..dim_x {
            if x > 10 && x < 100 && y > 5 && y < 20 {
                screen_buffer.insert((x, y), Color::WHITE);
            }
        }
    }
    screen_buffer
}
fn gen_full(dim_x: u16, dim_y: u16) -> HashMap<(u16, u16), Color> {
    let mut screen_buffer: HashMap<(u16, u16), Color> = HashMap::new();
    for y in 0..dim_y {
        for x in 0..dim_x {
            if x > 11 && x < 101 && y > 5 && y < 20 {
                screen_buffer.insert((x, y), Color::WHITE);
            }
        }
    }
    screen_buffer
}
