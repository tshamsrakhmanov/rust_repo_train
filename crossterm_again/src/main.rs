use crossterm::{
    QueueableCommand,
    cursor::{Hide, MoveTo, Show},
    event::{Event, KeyCode, poll, read},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode, size,
    },
};

use std::io::{self, Write};

pub mod pixel_generator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // set up refresh rate
    let fps: f32 = 1.0;
    let frame_time = 1000.0 / fps;

    // get terminal size to adapt for it
    let size = size()?;
    let x_dim = size.0 as f32;
    let y_dim = size.1 as f32;

    let x_scaling_factor = x_dim / 2.0;
    let y_scalling_factor = y_dim / 2.0;

    // set up all point which need to be drawn
    let mut points_cloud: Vec<(f32, f32)> = Vec::new();
    for pos in pixel_generator::frame_generator() {
        points_cloud.push(pos);
    }

    // set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;

    // main drawing loop
    'main_loop: loop {
        // Handle input
        if poll(std::time::Duration::from_millis(frame_time as u64))? {
            if let Event::Key(key_event) = read()? {
                if key_event.code == KeyCode::Char('q') {
                    break 'main_loop;
                }
            }
        }

        // draw frame
        draw_frame(
            &mut stdout,
            &points_cloud,
            &x_scaling_factor,
            &y_scalling_factor,
        )?;
    }

    // clean up terminal
    execute!(stdout, Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn draw_frame(
    stdout: &mut io::Stdout,
    points_cloud: &Vec<(f32, f32)>,
    x_scaling_factor: &f32,
    y_scaling_factor: &f32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Clear screen and draw new content
    stdout.queue(Clear(ClearType::All))?;

    for point in points_cloud {
        let modded_x_coordinate = x_scaling_factor * (point.0 + 1.0);
        let modded_y_coordinate = y_scaling_factor * (point.1 + 1.0);
        stdout.queue(MoveTo(
            modded_x_coordinate as u16,
            modded_y_coordinate as u16,
        ))?;
        stdout.queue(SetForegroundColor(Color::White))?;
        stdout.queue(Print("█"))?;
    }

    // Flush all commands at once
    stdout.flush()?;

    Ok(())
}
