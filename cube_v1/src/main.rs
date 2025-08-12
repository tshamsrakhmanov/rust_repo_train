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
use nalgebra::{Matrix4, Rotation3, Vector3, Vector4};
use std::f64::consts::PI;
use std::{
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
    let mut rotary: u8 = 0;

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

        if rotary > 36 {
            rotary = 0;
        } else {
            let mut points_cloud: Vec<(u32, u32)> = Vec::new();

            let edge: f64 = 7.0;

            let point1 = Vector4::new(edge * 2.0, edge * 2.0, edge, 1.0);
            let point2 = Vector4::new(-edge * 2.0, -edge * 2.0, edge, 1.0);
            let point3 = Vector4::new(-edge * 2.0, edge * 2.0, edge, 1.0);
            let point4 = Vector4::new(edge * 2.0, -edge * 2.0, edge * 2.0, 1.0);
            let point5 = Vector4::new(edge * 2.0, edge * 2.0, -edge, 1.0);
            let point6 = Vector4::new(-edge * 2.0, -edge * 2.0, -edge, 1.0);
            let point7 = Vector4::new(-edge * 2.0, edge * 2.0, -edge * 2.0, 1.0);
            let point8 = Vector4::new(edge * 2.0, -edge * 2.0, -edge, 1.0);
            //
            // let rotation_axis = Vector4::new(0.0, 1.0, 0.0, 0.0);
            // let axis_angle = Vector3::z() * f32::consts::FRAC_PI_8;
            // let rot = Rotation3::new(axis_angle);
            // let rotated_point = rot * Point3::new(point5.x, point5.y, point5.z);
            // println!("{rotated_point}");
            // std::thread::sleep(Duration::from_millis(10000));
            //
            let angle = rotary as f64 * 10.0;

            let poin1_rot = rotate_around_z(&point1, degrees_to_radians(angle));
            let poin2_rot = rotate_around_z(&point2, degrees_to_radians(angle));
            let poin3_rot = rotate_around_z(&point3, degrees_to_radians(angle));
            let poin4_rot = rotate_around_z(&point4, degrees_to_radians(angle));
            let poin5_rot = rotate_around_z(&point5, degrees_to_radians(angle));
            let poin6_rot = rotate_around_z(&point6, degrees_to_radians(angle));
            let poin7_rot = rotate_around_z(&point7, degrees_to_radians(angle));
            let poin8_rot = rotate_around_z(&point8, degrees_to_radians(angle));

            // let projection1 = screen_engine::calculate(x_dim, y_dim, point1);
            // let projection2 = screen_engine::calculate(x_dim, y_dim, point2);
            // let projection3 = screen_engine::calculate(x_dim, y_dim, point3);
            // let projection4 = screen_engine::calculate(x_dim, y_dim, point4);
            // let projection5 = screen_engine::calculate(x_dim, y_dim, point5);
            // let projection6 = screen_engine::calculate(x_dim, y_dim, point6);
            // let projection7 = screen_engine::calculate(x_dim, y_dim, point7);
            // let projection8 = screen_engine::calculate(x_dim, y_dim, point8);
            let projection1 = screen_engine::calculate(x_dim, y_dim, poin1_rot);
            let projection2 = screen_engine::calculate(x_dim, y_dim, poin2_rot);
            let projection3 = screen_engine::calculate(x_dim, y_dim, poin3_rot);
            let projection4 = screen_engine::calculate(x_dim, y_dim, poin4_rot);
            let projection5 = screen_engine::calculate(x_dim, y_dim, poin5_rot);
            let projection6 = screen_engine::calculate(x_dim, y_dim, poin6_rot);
            let projection7 = screen_engine::calculate(x_dim, y_dim, poin7_rot);
            let projection8 = screen_engine::calculate(x_dim, y_dim, poin8_rot);

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

            rotary += 1;
        }
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

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn rotate_around_z(point: &Vector4<f64>, angle_radians: f64) -> Vector4<f64> {
    // Create 3D rotation and convert to 4x4 homogeneous matrix
    let rotation = Rotation3::from_axis_angle(&Vector3::z_axis(), angle_radians);
    let rotation_matrix = Matrix4::from(rotation);

    rotation_matrix * point
}
