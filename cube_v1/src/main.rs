// use bresenham as br;
use core::f32;
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
use nalgebra::{Point3, Rotation3, Vector3, Vector4};
use std::{
    f32,
    io::{self, Write},
    time::Duration,
};

mod screen_engine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::thread::sleep(Duration::from_millis(100));
    // setup frame time
    let fps: f32 = 25.0;
    let frame_time = 1000.0 / fps;

    // setup screen size
    let size = size()?;
    let x_dim = size.0 as f64;
    let y_dim = size.1 as f64;

    // part of main loop where we calculate points on the screen

    // let mut points_cloud: Vec<(u32, u32)> = Vec::new();
    //
    // let edge: f64 = 10.0;
    //
    // let point1 = Vector4::new(edge, edge, edge, 1.0);
    // let point2 = Vector4::new(-edge, -edge, edge, 1.0);
    // let point3 = Vector4::new(-edge, edge, edge, 1.0);
    // let point4 = Vector4::new(edge, -edge, edge, 1.0);
    // let point5 = Vector4::new(edge, edge, -edge, 1.0);
    // let point6 = Vector4::new(-edge, -edge, -edge, 1.0);
    // let point7 = Vector4::new(-edge, edge, -edge, 1.0);
    // let point8 = Vector4::new(edge, -edge, -edge, 1.0);
    //
    // // let rotation_axis = Vector4::new(0.0, 1.0, 0.0, 0.0);
    // // let axis_angle = Vector3::z() * f32::consts::FRAC_PI_8;
    // // let rot = Rotation3::new(axis_angle);
    // // let rotated_point = rot * Point3::new(point5.x, point5.y, point5.z);
    // // println!("{rotated_point}");
    // // std::thread::sleep(Duration::from_millis(10000));
    //
    // let projection1 = screen_engine::calculate(x_dim, y_dim, point1);
    // let projection2 = screen_engine::calculate(x_dim, y_dim, point2);
    // let projection3 = screen_engine::calculate(x_dim, y_dim, point3);
    // let projection4 = screen_engine::calculate(x_dim, y_dim, point4);
    // let projection5 = screen_engine::calculate(x_dim, y_dim, point5);
    // let projection6 = screen_engine::calculate(x_dim, y_dim, point6);
    // let projection7 = screen_engine::calculate(x_dim, y_dim, point7);
    // let projection8 = screen_engine::calculate(x_dim, y_dim, point8);
    //
    // points_cloud.push(projection1);
    // points_cloud.push(projection2);
    // points_cloud.push(projection3);
    // points_cloud.push(projection4);
    // points_cloud.push(projection5);
    // points_cloud.push(projection6);
    // points_cloud.push(projection7);
    // points_cloud.push(projection8);
    //
    // prepare variable to store rotation factor
    let mut factor: u8 = 0;
    let axis_angle = Vector3::z() * f32::consts::FRAC_PI_8;
    let rot = Rotation3::new(axis_angle);

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

        let mut points_cloud: Vec<(u32, u32)> = Vec::new();

        let edge: f64 = 10.0;

        let point1 = Vector4::new(edge, edge, edge, 1.0);
        let point2 = Vector4::new(-edge, -edge, edge, 1.0);
        let point3 = Vector4::new(-edge, edge, edge, 1.0);
        let point4 = Vector4::new(edge, -edge, edge, 1.0);
        let point5 = Vector4::new(edge, edge, -edge, 1.0);
        let point6 = Vector4::new(-edge, -edge, -edge, 1.0);
        let point7 = Vector4::new(-edge, edge, -edge, 1.0);
        let point8 = Vector4::new(edge, -edge, -edge, 1.0);

        // let rotation_axis = Vector4::new(0.0, 1.0, 0.0, 0.0);
        // let axis_angle = Vector3::z() * f32::consts::FRAC_PI_8;
        // let rot = Rotation3::new(axis_angle);
        // let rotated_point = rot * Point3::new(point5.x, point5.y, point5.z);
        // println!("{rotated_point}");
        // std::thread::sleep(Duration::from_millis(10000));

        let rot = Rotation3::from_axis_angle(&Vector3::z_axis(), f32::consts::FRAC_PI_6);
        let rot_point = rot * point1;

        let projection1 = screen_engine::calculate(x_dim, y_dim, point1);
        let projection2 = screen_engine::calculate(x_dim, y_dim, point2);
        let projection3 = screen_engine::calculate(x_dim, y_dim, point3);
        let projection4 = screen_engine::calculate(x_dim, y_dim, point4);
        let projection5 = screen_engine::calculate(x_dim, y_dim, point5);
        let projection6 = screen_engine::calculate(x_dim, y_dim, point6);
        let projection7 = screen_engine::calculate(x_dim, y_dim, point7);
        let projection8 = screen_engine::calculate(x_dim, y_dim, point8);

        points_cloud.push(projection1);
        points_cloud.push(projection2);
        points_cloud.push(projection3);
        points_cloud.push(projection4);
        points_cloud.push(projection5);
        points_cloud.push(projection6);
        points_cloud.push(projection7);
        points_cloud.push(projection8);

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
