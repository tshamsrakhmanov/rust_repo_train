use bresenham as br;
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
use nalgebra::{Matrix4, Point3, Point4, Rotation3, Vector3, Vector4};
use std::f64::consts::PI;
use std::io::{self, Write};

mod screen_engine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup frame time
    let fps: f32 = 20.0;
    let frame_time = 1000.0 / fps;

    // setup screen size
    let size = size()?;
    let x_dim = size.0 as f64;
    let y_dim = size.1 as f64;

    // prepare variable to store rotation factor
    let mut rotary: u8 = 0;
    // variable to count rotation angle
    let angler: u8 = 36;

    // prepare ObjectCube
    let p = Point3::origin();
    let edge: f64 = 10.0;
    let cube = ObjectCube::new_from_center(p, edge);
    let ps = cube.get_points();
    let ps2 = cube.get_points();

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

        if rotary > angler {
            rotary = 0;
        }

        let mut points_cloud: Vec<(u32, u32)> = Vec::new();

        let edge: f64 = 10.0;

        let point1 = Vector4::new(edge * 2.0, edge * 2.0, edge, 1.0);
        let point2 = Vector4::new(-edge * 2.0, -edge * 2.0, edge, 1.0);
        let point3 = Vector4::new(-edge * 2.0, edge * 2.0, edge, 1.0);
        let point4 = Vector4::new(edge * 2.0, -edge * 2.0, edge * 1.0, 1.0);
        let point5 = Vector4::new(edge * 2.0, edge * 2.0, -edge, 1.0);
        let point6 = Vector4::new(-edge * 2.0, -edge * 2.0, -edge, 1.0);
        let point7 = Vector4::new(-edge * 2.0, edge * 2.0, -edge * 1.0, 1.0);
        let point8 = Vector4::new(edge * 2.0, -edge * 2.0, -edge, 1.0);

        let angle = rotary as f64 * 10.0;

        let poin1_rot = rotate_around_z(&point1, degrees_to_radians(angle));
        let poin2_rot = rotate_around_z(&point2, degrees_to_radians(angle));
        let poin3_rot = rotate_around_z(&point3, degrees_to_radians(angle));
        let poin4_rot = rotate_around_z(&point4, degrees_to_radians(angle));
        let poin5_rot = rotate_around_z(&point5, degrees_to_radians(angle));
        let poin6_rot = rotate_around_z(&point6, degrees_to_radians(angle));
        let poin7_rot = rotate_around_z(&point7, degrees_to_radians(angle));
        let poin8_rot = rotate_around_z(&point8, degrees_to_radians(angle));

        let projection1 = screen_engine::calculate(x_dim, y_dim, poin1_rot);
        let prj1_unwrap = (projection1.0 as isize, projection1.1 as isize);
        let projection2 = screen_engine::calculate(x_dim, y_dim, poin2_rot);
        let prj2_unwrap = (projection2.0 as isize, projection2.1 as isize);
        let projection3 = screen_engine::calculate(x_dim, y_dim, poin3_rot);
        let prj3_unwrap = (projection3.0 as isize, projection3.1 as isize);
        let projection4 = screen_engine::calculate(x_dim, y_dim, poin4_rot);
        let prj4_unwrap = (projection4.0 as isize, projection4.1 as isize);
        let projection5 = screen_engine::calculate(x_dim, y_dim, poin5_rot);
        let prj5_unwrap = (projection5.0 as isize, projection5.1 as isize);
        let projection6 = screen_engine::calculate(x_dim, y_dim, poin6_rot);
        let prj6_unwrap = (projection6.0 as isize, projection6.1 as isize);
        let projection7 = screen_engine::calculate(x_dim, y_dim, poin7_rot);
        let prj7_unwrap = (projection7.0 as isize, projection7.1 as isize);
        let projection8 = screen_engine::calculate(x_dim, y_dim, poin8_rot);
        let prj8_unwrap = (projection8.0 as isize, projection8.1 as isize);

        let line1 = br::Bresenham::new(prj1_unwrap, prj3_unwrap);
        let line2 = br::Bresenham::new(prj2_unwrap, prj3_unwrap);
        let line3 = br::Bresenham::new(prj4_unwrap, prj1_unwrap);
        let line4 = br::Bresenham::new(prj4_unwrap, prj2_unwrap);
        let line5 = br::Bresenham::new(prj1_unwrap, prj5_unwrap);
        let line6 = br::Bresenham::new(prj2_unwrap, prj6_unwrap);
        let line7 = br::Bresenham::new(prj3_unwrap, prj7_unwrap);
        let line8 = br::Bresenham::new(prj4_unwrap, prj8_unwrap);
        let line9 = br::Bresenham::new(prj5_unwrap, prj7_unwrap);
        let line10 = br::Bresenham::new(prj6_unwrap, prj7_unwrap);
        let line11 = br::Bresenham::new(prj8_unwrap, prj5_unwrap);
        let line12 = br::Bresenham::new(prj8_unwrap, prj6_unwrap);

        points_cloud.push(projection1);
        points_cloud.push(projection2);
        points_cloud.push(projection3);
        points_cloud.push(projection4);
        points_cloud.push(projection5);
        points_cloud.push(projection6);
        points_cloud.push(projection7);
        points_cloud.push(projection8);
        for pos in line1 {
            points_cloud.push((pos.0 as u32, pos.1 as u32));
        }
        for pos in line2 {
            points_cloud.push((pos.0 as u32, pos.1 as u32));
        }
        for pos in line3 {
            points_cloud.push((pos.0 as u32, pos.1 as u32));
        }
        for pos in line4 {
            points_cloud.push((pos.0 as u32, pos.1 as u32));
        }
        for pos in line5 {
            points_cloud.push((pos.0 as u32, pos.1 as u32));
        }
        for pos in line6 {
            points_cloud.push((pos.0 as u32, pos.1 as u32));
        }
        for pos in line7 {
            points_cloud.push((pos.0 as u32, pos.1 as u32));
        }
        for pos in line8 {
            points_cloud.push((pos.0 as u32, pos.1 as u32));
        }
        for pos in line9 {
            points_cloud.push((pos.0 as u32, pos.1 as u32));
        }
        for pos in line10 {
            points_cloud.push((pos.0 as u32, pos.1 as u32));
        }
        for pos in line11 {
            points_cloud.push((pos.0 as u32, pos.1 as u32));
        }
        for pos in line12 {
            points_cloud.push((pos.0 as u32, pos.1 as u32));
        }

        // draw frame
        draw_frame(&mut stdout, &points_cloud)?;

        rotary += 1;
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

struct ObjectCube {
    points: Vec<Vector4<f64>>,
}

impl ObjectCube {
    fn new_from_center(cp: Point3<f64>, e: f64) -> ObjectCube {
        let mut points: Vec<Vector4<f64>> = Vec::new();
        let he = e / 2.0; // he - half edge
        let p1 = Vector4::new(cp.x + he, cp.y + he, cp.z + he, 1.0);
        let p2 = Vector4::new(cp.x - he, cp.y - he, cp.z + he, 1.0);
        let p3 = Vector4::new(cp.x - he, cp.y + he, cp.z + he, 1.0);
        let p4 = Vector4::new(cp.x + he, cp.y - he, cp.z + he, 1.0);
        let p5 = Vector4::new(cp.x + he, cp.y + he, cp.z - he, 1.0);
        let p6 = Vector4::new(cp.x - he, cp.y - he, cp.z - he, 1.0);
        let p7 = Vector4::new(cp.x - he, cp.y + he, cp.z - he, 1.0);
        let p8 = Vector4::new(cp.x + he, cp.y - he, cp.z - he, 1.0);
        points.push(p1);
        points.push(p2);
        points.push(p3);
        points.push(p4);
        points.push(p5);
        points.push(p6);
        points.push(p7);
        points.push(p8);
        let r = ObjectCube { points: points };

        r
    }
    fn get_points(&self) -> &Vec<Vector4<f64>> {
        &self.points
    }
}
