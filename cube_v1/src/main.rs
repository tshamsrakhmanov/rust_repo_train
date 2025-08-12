// use bresenham as br;
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
use nalgebra::Vector4;
use std::{
    io::{self, Write},
    os::unix::thread,
    time::Duration,
};

mod screen_engine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup frame time
    let fps: f32 = 1.0;
    let frame_time = 1000.0 / fps;

    // setup screen size
    let size = size()?;
    let x_dim = size.0 as f64;
    let y_dim = size.1 as f64;

    // part of main loop where we calculate points on the screen

    let mut points_cloud: Vec<(u32, u32)> = Vec::new();

    let point1 = Vector4::new(0.0, 0.0, 0.0, 1.0);
    let point2 = Vector4::new(20.0, 0.0, 0.0, 1.0);
    let point3 = Vector4::new(0.0, 20.0, 0.0, 1.0);
    let point4 = Vector4::new(0.0, 0.0, 10.0, 1.0);

    let rotation_axis = Vector4::new(0.0, 1.0, 0.0, 0.0);

    let projection1 = screen_engine::calculate(x_dim, y_dim, point1);
    let projection2 = screen_engine::calculate(x_dim, y_dim, point2);
    let projection3 = screen_engine::calculate(x_dim, y_dim, point3);
    let projection4 = screen_engine::calculate(x_dim, y_dim, point4);

    points_cloud.push(projection1);
    points_cloud.push(projection2);
    points_cloud.push(projection3);
    points_cloud.push(projection4);

    // prepare rotation axis - Y

    // prepare drawing engine
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
        draw_frame(&mut stdout, &points_cloud)?;
    }

    // clean up terminal
    execute!(stdout, Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn draw_frame(
    stdout: &mut io::Stdout,
    points_cloud: &Vec<(u32, u32)>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Clear screen and draw new content
    stdout.queue(Clear(ClearType::All))?;

    for point in points_cloud {
        stdout.queue(MoveTo(point.0 as u16, point.1 as u16))?;
        stdout.queue(SetForegroundColor(Color::White))?;
        stdout.queue(Print("█"))?;
    }

    // Flush all commands at once
    stdout.flush()?;

    Ok(())
}
