use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEvent, poll, read},
    execute,
    style::Print,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode, size,
    },
};
use std::io::{self, Write};
use std::time::Duration;
fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    let screen_size = size()?;
    let dim_x = screen_size.0;
    let dim_y = screen_size.1;

    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, cursor::Hide)?;

    'main_loop: loop {
        if poll(Duration::from_millis(60))? {
            if let Event::Key(event) = read()? {
                if event.code == KeyCode::Char('q') {
                    break 'main_loop;
                }
            }
        }

        execute!(stdout, cursor::MoveTo(10, 4))?;
        execute!(stdout, Print('a'))?;
        stdout.flush()?;
    }

    disable_raw_mode()?;
    execute!(stdout, cursor::Show)?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

struct Pixel {
    x: u16,
    y: u16,
    color: Color,
}
impl Pixel {
    fn new(x: u16, y: u16, color: Color) -> Pixel {
        let answer = Pixel {
            x: x,
            y: y,
            color: color,
        };
        answer
    }
}

struct ScreenBuffer {
    buffer: Vec<Pixel>,
    dim_x: u16,
    dim_y: u16,
}

impl ScreenBuffer {
    fn new(pixels: Vec<Pixel>, dim_x: u16, dim_y: u16) -> ScreenBuffer {
        let answer = ScreenBuffer {
            buffer: pixels,
            dim_x: dim_x,
            dim_y: dim_y,
        };
        answer
    }
    fn get_drawing_string(&self) -> String {
        let answer = String::new();

        for y in 0..self.dim_y {
            for x in 0..self.dim_x {
                let temp_pixel = Pixel::new(x, y, Option::Some(Color));
            }
        }

        answer
    }
}

enum Color {
    WHITE,
    BLUE,
    RED,
}
