#[warn(unused_imports)]
use crossterm::{
    QueueableCommand,
    cursor::{Hide, MoveTo, Show, position},
    event::{Event, KeyCode, poll, read},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode, size,
    },
};

pub mod pixel_generator;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let size = size()?;
    let x_dim = size.0;
    let y_dim = size.1;

    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;

    // set up all point which need to be drawn
    let mut points_cloud: Vec<(u16, u16)> = Vec::new();
    for pos in pixel_generator::frame_generator(x_dim, y_dim) {
        points_cloud.push(pos);
    }

    // Main drawing loop
    'main_loop: loop {
        // Handle input
        if poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key_event) = read()? {
                if key_event.code == KeyCode::Char('q') {
                    break 'main_loop;
                }
            }
        }

        // Draw frame
        draw_frame(&mut stdout, &points_cloud)?;
        // std::thread::sleep(Duration::from_millis(10));
    }

    // Clean up terminal
    execute!(stdout, Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn draw_frame(
    stdout: &mut io::Stdout,
    points_cloud: &Vec<(u16, u16)>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Clear screen and draw new content
    stdout.queue(Clear(ClearType::All))?;

    for point in points_cloud {
        stdout.queue(MoveTo(point.0, point.1))?;
        stdout.queue(SetForegroundColor(Color::White))?;
        stdout.queue(Print("O"))?;
    }

    // Flush all commands at once
    stdout.flush()?;

    Ok(())
}
