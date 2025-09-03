use bresenham;
use crossterm::{
    cursor::{self, Hide, Show},
    event::{Event, KeyCode, poll, read},
    execute,
    style::{self, PrintStyledContent, Stylize},
    terminal::{
        self, Clear, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};
use nalgebra::{Matrix4, Point3, Rotation3, Vector3, Vector4};
use std::io::stdout;
use std::time::Duration;
use std::{
    f64::consts::PI,
    io::{self},
};

fn main() -> io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, cursor::Hide);
    execute!(stdout, EnterAlternateScreen)?;
    execute!(stdout, Clear(terminal::ClearType::All));
    enable_raw_mode();

    let edge = 20.0;
    let p0 = Vector4::new(0.0, -edge, edge, 1.0);
    let p1 = Vector4::new(0.0, edge, edge, 1.0);
    let p2 = Vector4::new(-edge, 0.0, -edge, 1.0);
    let p3 = Vector4::new(edge, 0.0, -edge, 1.0);

    let pov_position = Vector4::new(10.0, 10.0, 5.0, 0.0);

    let mut pyramid0 = PyramidV4 {
        point0: p0,
        point1: p1,
        point2: p2,
        point3: p3,
    };

    let screen = terminal::size()?;
    let dim_x = screen.0;
    let dim_y = screen.1;

    let left = screen.0 as f64 / -2.0;
    let right = screen.0 as f64 / 2.0;
    let bottom = screen.1 as f64 / -1.0;
    let top = screen.1 as f64 / 1.0;
    let znear = 0.0;
    let zfar = 1.0;

    let projection = Matrix4::new_orthographic(left, right, bottom, top, znear, zfar);

    let eye = Point3::new(pov_position.x, pov_position.y, pov_position.z);
    let target = Point3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 0.0, 1.0);

    let view_matrix = Matrix4::look_at_rh(&eye, &target, &up);

    let model_matrix = Matrix4::identity();

    let pvm_matrix = projection * view_matrix * model_matrix;

    'main_loop: loop {
        if poll(Duration::from_millis(30))? {
            if let Event::Key(event) = read()? {
                if event.code == KeyCode::Char('q') {
                    break 'main_loop;
                }
            }
        }
        execute!(stdout, Clear(terminal::ClearType::All))?;
        pyramid0.rotate_full_mut(7.0);

        for triangle in pyramid0.get_triangles() {
            if triangle.is_visible(pov_position) {
                let projection = triangle.projection(pvm_matrix, dim_x as f64, dim_y as f64);
                let line0 = bresenham::Bresenham::new(
                    (projection.point0.0 as isize, projection.point0.1 as isize),
                    (projection.point1.0 as isize, projection.point1.1 as isize),
                );
                let line1 = bresenham::Bresenham::new(
                    (projection.point1.0 as isize, projection.point1.1 as isize),
                    (projection.point2.0 as isize, projection.point2.1 as isize),
                );
                let line2 = bresenham::Bresenham::new(
                    (projection.point2.0 as isize, projection.point2.1 as isize),
                    (projection.point0.0 as isize, projection.point0.1 as isize),
                );
                for pos in line0 {
                    execute!(stdout, cursor::MoveTo(pos.0 as u16, pos.1 as u16))?;
                    execute!(stdout, PrintStyledContent("0".magenta()))?;
                }
                for pos in line1 {
                    execute!(stdout, cursor::MoveTo(pos.0 as u16, pos.1 as u16))?;
                    execute!(stdout, PrintStyledContent("0".magenta()))?;
                }
                for pos in line2 {
                    execute!(stdout, cursor::MoveTo(pos.0 as u16, pos.1 as u16))?;
                    execute!(stdout, PrintStyledContent("0".magenta()))?;
                }
            }
        }
    }

    execute!(stdout, Clear(terminal::ClearType::All))?;
    disable_raw_mode()?;
    execute!(stdout, cursor::Show)?;
    execute!(io::stdout(), LeaveAlternateScreen)
}

struct PyramidV4 {
    point0: Vector4<f64>,
    point1: Vector4<f64>,
    point2: Vector4<f64>,
    point3: Vector4<f64>,
}

impl PyramidV4 {
    fn get_triangles(&self) -> Vec<TriangleV4> {
        let mut answer: Vec<TriangleV4> = Vec::new();
        let tr0 = TriangleV4 {
            point0: self.point0,
            point1: self.point2,
            point2: self.point1,
        };
        let tr1 = TriangleV4 {
            point0: self.point0,
            point1: self.point1,
            point2: self.point3,
        };
        let tr2 = TriangleV4 {
            point0: self.point0,
            point1: self.point3,
            point2: self.point2,
        };
        let tr3 = TriangleV4 {
            point0: self.point1,
            point1: self.point2,
            point2: self.point3,
        };
        answer.push(tr0);
        answer.push(tr1);
        answer.push(tr2);
        answer.push(tr3);
        answer
    }
    fn rotate_full_mut(&mut self, angle_deg: f64) {
        self.point0 = rotate_point_by_axis_z(self.point0, angle_deg);
        self.point1 = rotate_point_by_axis_z(self.point1, angle_deg);
        self.point2 = rotate_point_by_axis_z(self.point2, angle_deg);
        self.point3 = rotate_point_by_axis_z(self.point3, angle_deg);
    }
}

#[derive(Debug)]
struct TriangleV4 {
    point0: Vector4<f64>,
    point1: Vector4<f64>,
    point2: Vector4<f64>,
}

impl TriangleV4 {
    fn is_visible(&self, pov_vector: Vector4<f64>) -> bool {
        let v0 = Vector3::new(
            self.point1.x - self.point0.x,
            self.point1.y - self.point0.y,
            self.point1.z - self.point0.z,
        );
        let v1 = Vector3::new(
            self.point2.x - self.point0.x,
            self.point2.y - self.point0.y,
            self.point2.z - self.point0.z,
        );

        let norm = v0.cross(&v1);

        let pov_vec_to_v3: Vector3<f64> = Vector3::new(pov_vector.x, pov_vector.y, pov_vector.z);

        let angle = norm.angle(&pov_vec_to_v3);
        let angle_to_deg = angle * 180.0 / PI;

        if angle_to_deg < 90.0 { true } else { false }
    }

    fn projection(&self, pvm_matrix: Matrix4<f64>, dim_x: f64, dim_y: f64) -> TriangleV3 {
        let p0_raw = pvm_matrix * self.point0;
        let p1_raw = pvm_matrix * self.point1;
        let p2_raw = pvm_matrix * self.point2;

        let p0x = (dim_x / 2.0) * (1.0 + p0_raw.x);
        let p1x = (dim_x / 2.0) * (1.0 + p1_raw.x);
        let p2x = (dim_x / 2.0) * (1.0 + p2_raw.x);

        let p0y = (dim_y / 2.0) * (1.0 + p0_raw.y);
        let p1y = (dim_y / 2.0) * (1.0 + p1_raw.y);
        let p2y = (dim_y / 2.0) * (1.0 + p2_raw.y);

        let a = TriangleV3 {
            point0: (p0x as u16, p0y as u16),
            point1: (p1x as u16, p1y as u16),
            point2: (p2x as u16, p2y as u16),
        };
        a
    }
}

#[derive(Debug)]
struct TriangleV3 {
    point0: (u16, u16),
    point1: (u16, u16),
    point2: (u16, u16),
}

fn rotate_point_by_axis_z(point: Vector4<f64>, angle_deg: f64) -> Vector4<f64> {
    let rotation = Rotation3::from_axis_angle(&Vector3::z_axis(), angle_deg * PI / 180.0);
    let point_to_v3 = Vector3::new(point.x, point.y, point.z);
    let answer_v3 = rotation * point_to_v3;
    let answer_v4 = Vector4::new(answer_v3.x, answer_v3.y, answer_v3.z, 1.0);
    answer_v4
}
