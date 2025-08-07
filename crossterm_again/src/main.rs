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
use std::{
    io::{self, Write},
    time::Duration,
};

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

    // fill points cloud with some data
    for i in 0..x_dim {
        for j in 0..y_dim {
            //..
            if i == 0 {
                points_cloud.push((0, j));
            }
            if j == 0 {
                points_cloud.push((i, 0));
            }
            if i == x_dim - 1 {
                points_cloud.push((x_dim - 1, j));
            }
            if j == y_dim - 1 {
                points_cloud.push((i, y_dim - 1));
            }
        }
        //..
    }

    let line_vertical_down = brezenhamm(20, 20, 20, 25);
    let line_vertical_up = brezenhamm(20, 20, 20, 15);
    let line_horizontal_left = brezenhamm(20, 20, 25, 20);
    let line_horizontal_right = brezenhamm(20, 20, 15, 20);

    for pos in line_vertical_down {
        points_cloud.push(pos);
    }
    for pos in line_vertical_up {
        points_cloud.push(pos);
    }
    for pos in line_horizontal_left {
        points_cloud.push(pos);
    }
    for pos in line_horizontal_right {
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

fn brezenhamm(x0: u16, y0: u16, x1: u16, y1: u16) -> Vec<(u16, u16)> {
    let mut answer: Vec<(u16, u16)> = Vec::new();

    // point case
    if x0 == x1 && y0 == y1 {
        answer.push((x0, y0));
        //..
    }

    // straight line cases
    if x0 == x1 && y0 > y1 {
        for i in y1..y0 {
            answer.push((x0, i));
        }
    }
    if x0 == x1 && y1 > y0 {
        for i in y0..y1 {
            answer.push((x0, i));
        }
    }
    if y0 == y1 && x0 > x1 {
        for i in x1..x0 {
            answer.push((i, y0));
        }
    }
    if y0 == y1 && x1 > x0 {
        for i in x0..x1 {
            answer.push((i, y0));
        }
    }

    answer
}
