use bresenham::Bresenham as br;
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
use nalgebra::{Matrix4, Point3, Rotation3, Vector3, Vector4};
use std::f64::consts::PI;
use std::io::{self, Write};

mod screen_engine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup frame time
    let fps: f32 = 30.0;
    let frame_time = 1000.0 / fps;

    // setup screen size
    let size = size()?;
    let x_dim = size.0 as f64;
    let y_dim = size.1 as f64;

    // prepare variable to store rotation factor
    let mut rotary: u16 = 0;
    // variable to count rotation angle
    let angler: u16 = 360;

    // prepare ObjectCube
    let p = Point3::origin();
    let edge: f64 = 100.0;
    let cube = ObjectCube::new_from_center(p, edge);
    let ps = cube.get_points();

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

        // whole mess with cube points and their rotation
        //
        //
        //
        if rotary > angler {
            rotary = 0;
        }

        let mut p_buff: Vec<(u16, u16)> = Vec::new();

        let e: f64 = 15.0; // edge of a cube

        let a = rotary as f64 * 1.0; // angle - to use as rotation step

        let p1rot = rot_z(&Vector4::new(e, e, e, 1.0), deg_to_rad(a));
        let p5rot = rot_z(&Vector4::new(e, e, -e, 1.0), deg_to_rad(a));
        let p4rot = rot_z(&Vector4::new(e, -e, e, 1.0), deg_to_rad(a));
        let p8rot = rot_z(&Vector4::new(e, -e, -e, 1.0), deg_to_rad(a));
        let p3rot = rot_z(&Vector4::new(-e, e, e, 1.0), deg_to_rad(a));
        let p7rot = rot_z(&Vector4::new(-e, e, -e, 1.0), deg_to_rad(a));
        let p2rot = rot_z(&Vector4::new(-e, -e, e, 1.0), deg_to_rad(a));
        let p6rot = rot_z(&Vector4::new(-e, -e, -e, 1.0), deg_to_rad(a));

        let prj1 = screen_engine::calc(x_dim, y_dim, p1rot);
        let prj2 = screen_engine::calc(x_dim, y_dim, p2rot);
        let prj3 = screen_engine::calc(x_dim, y_dim, p3rot);
        let prj4 = screen_engine::calc(x_dim, y_dim, p4rot);
        let prj5 = screen_engine::calc(x_dim, y_dim, p5rot);
        let prj6 = screen_engine::calc(x_dim, y_dim, p6rot);
        let prj7 = screen_engine::calc(x_dim, y_dim, p7rot);
        let prj8 = screen_engine::calc(x_dim, y_dim, p8rot);

        let line1 = br::new(prj1, prj3);
        let line2 = br::new(prj2, prj3);
        let line3 = br::new(prj4, prj1);
        let line4 = br::new(prj4, prj2);
        let line5 = br::new(prj1, prj5);
        let line6 = br::new(prj2, prj6);
        let line7 = br::new(prj3, prj7);
        let line8 = br::new(prj4, prj8);
        let line9 = br::new(prj5, prj7);
        let line10 = br::new(prj6, prj7);
        let line11 = br::new(prj8, prj5);
        let line12 = br::new(prj8, prj6);

        p_buff.push((prj1.0 as u16, prj1.1 as u16));
        p_buff.push((prj2.0 as u16, prj2.1 as u16));
        p_buff.push((prj3.0 as u16, prj3.1 as u16));
        p_buff.push((prj4.0 as u16, prj4.1 as u16));
        p_buff.push((prj5.0 as u16, prj5.1 as u16));
        p_buff.push((prj6.0 as u16, prj6.1 as u16));
        p_buff.push((prj7.0 as u16, prj7.1 as u16));
        p_buff.push((prj8.0 as u16, prj8.1 as u16));

        for pos in line1 {
            p_buff.push((pos.0 as u16, pos.1 as u16));
        }
        for pos in line2 {
            p_buff.push((pos.0 as u16, pos.1 as u16));
        }
        for pos in line3 {
            p_buff.push((pos.0 as u16, pos.1 as u16));
        }
        for pos in line4 {
            p_buff.push((pos.0 as u16, pos.1 as u16));
        }
        for pos in line5 {
            p_buff.push((pos.0 as u16, pos.1 as u16));
        }
        for pos in line6 {
            p_buff.push((pos.0 as u16, pos.1 as u16));
        }
        for pos in line7 {
            p_buff.push((pos.0 as u16, pos.1 as u16));
        }
        for pos in line8 {
            p_buff.push((pos.0 as u16, pos.1 as u16));
        }
        for pos in line9 {
            p_buff.push((pos.0 as u16, pos.1 as u16));
        }
        for pos in line10 {
            p_buff.push((pos.0 as u16, pos.1 as u16));
        }
        for pos in line11 {
            p_buff.push((pos.0 as u16, pos.1 as u16));
        }
        for pos in line12 {
            p_buff.push((pos.0 as u16, pos.1 as u16));
        }

        // mess ends here

        // draw frame
        draw_frame(&mut stdout, &p_buff)?;

        rotary += 1;
    }

    // clean up terminal
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
        stdout.queue(Print("█"))?;
    }

    // Flush all commands at once
    stdout.flush()?;

    Ok(())
}

fn deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn rot_z(point: &Vector4<f64>, angle_radians: f64) -> Vector4<f64> {
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
