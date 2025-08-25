use std::{
    io::{Write, stdout},
    time::Duration,
};

use crossterm::{
    QueueableCommand, cursor,
    event::{self, Event, KeyCode, poll, read},
    execute,
    style::Print,
    terminal::{
        self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use nalgebra::{Matrix4, Point3, Vector3, Vector4};
use std::io;
fn main() -> io::Result<()> {
    let mut stdout = stdout();

    let screen_size = terminal::size()?;
    let dim_x = screen_size.0;
    let dim_y = screen_size.1;

    let left = -(dim_x as f64) / 2.0;
    let right = (dim_x as f64) / 2.0;
    let bottom = -(dim_y as f64) / 2.0;
    let top = (dim_y as f64) / 2.0;
    let znear = 0.0;
    let zfar = 100.0;

    let project_matrix = nalgebra::Matrix4::new_orthographic(left, right, bottom, top, znear, zfar);

    let view_x: f64 = 10.0;
    let view_y: f64 = 10.0;
    let view_z: f64 = 10.0;

    let eye = Point3::new(view_x, view_y, view_z);
    let target = Point3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 0.0, -1.0);

    let view_matrix = nalgebra::Matrix4::look_at_rh(&eye, &target, &up);

    let model_matrix: Matrix4<f64> = nalgebra::Matrix4::identity();

    let pvm = project_matrix * view_matrix * model_matrix;

    let point0 = Vector4::new(0.0, 0.0, 0.0, 1.0);
    let point1 = Vector4::new(10.0, 0.0, 0.0, 1.0);
    let point2 = Vector4::new(0.0, 10.0, 0.0, 1.0);
    let point3 = Vector4::new(0.0, 0.0, 10.0, 1.0);
    let pyr0 = PyramidV4::new(point0, point1, point2, point3);

    execute!(stdout, cursor::Hide)?;
    execute!(stdout, EnterAlternateScreen)?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    let mut screen_buffer: Vec<(u16, u16)> = Vec::new();
    screen_buffer.push((15, 15));

    for pos in pyr0.get_triangles() {
        println!("{pos:?\n}");
        for pos in pos.project_to_screen(pvm).resterize() {
            println!("{pos}");
        }
    }

    std::thread::sleep(Duration::from_millis(40000));

    enable_raw_mode()?;
    'main_loop: loop {
        stdout.queue(Clear(ClearType::All))?;
        if poll(Duration::from_millis(100))? {
            if let Event::Key(event) = read()? {
                if event.code == KeyCode::Char('q') {
                    break 'main_loop;
                }
            }
        }

        for pos in &screen_buffer {
            execute!(stdout, cursor::MoveTo(pos.0, pos.1))?;
            execute!(stdout, Print('O'))?;
        }
        stdout.flush()?;
    }

    disable_raw_mode()?;
    execute!(io::stdout(), cursor::Show)?;
    execute!(io::stdout(), LeaveAlternateScreen)
}

#[derive(Debug)]
struct PyramidV4 {
    point0: Vector4<f64>,
    point1: Vector4<f64>,
    point2: Vector4<f64>,
    point3: Vector4<f64>,
}

#[derive(Debug)]
struct TriangleV4 {
    point0: Vector4<f64>,
    point1: Vector4<f64>,
    point2: Vector4<f64>,
}

#[derive(Debug)]
struct TriangleV3 {
    point0: Vector3<f64>,
    point1: Vector3<f64>,
    point2: Vector3<f64>,
}

impl TriangleV3 {
    fn new(point0: Vector3<f64>, point1: Vector3<f64>, point2: Vector3<f64>) -> TriangleV3 {
        let answer = TriangleV3 {
            point0: point0,
            point1: point1,
            point2: point2,
        };
        answer
    }
    fn resterize(&self) -> Vec<(u16, u16)> {
        let answer: Vec<(u16, u16)> = Vec::new();
        l

        answer
    }
}

impl TriangleV4 {
    fn new(point0: Vector4<f64>, point1: Vector4<f64>, point2: Vector4<f64>) -> TriangleV4 {
        let a = TriangleV4 {
            point0: point0,
            point1: point1,
            point2: point2,
        };
        a
    }
    fn get_normal(&self) -> Vector4<f64> {
        let v1 = Vector4::new(
            self.point1.x - self.point0.x,
            self.point1.y - self.point0.y,
            self.point1.z - self.point0.z,
            1.0,
        );

        let v2 = Vector4::new(
            self.point2.x - self.point0.x,
            self.point2.y - self.point0.y,
            self.point2.z - self.point0.z,
            1.0,
        );
        let cross = v1.cross(&v2);
        cross
    }
    fn is_visible(&self, vector: &Vector4<f64>) -> bool {
        let mut answer = false;
        let normal = self.get_normal();
        let angle = normal.angle(vector);
        if angle > 90.0 {
            answer = true;
        }
        answer
    }
    fn project_to_screen(&self, pvm: Matrix4<f64>) -> TriangleV3 {
        let p0 = pvm * self.point0;
        let p1 = pvm * self.point1;
        let p2 = pvm * self.point2;

        let p0: Vector3<f64> = Vector3::new(p0.x, p0.y, 0.0);
        let p1: Vector3<f64> = Vector3::new(p1.x, p1.y, 0.0);
        let p2: Vector3<f64> = Vector3::new(p2.x, p2.y, 0.0);

        let a = TriangleV3 {
            point0: p0,
            point1: p1,
            point2: p2,
        };
        a
    }
}

impl PyramidV4 {
    fn get_triangles(&self) -> Vec<TriangleV4> {
        let mut answer: Vec<TriangleV4> = Vec::new();
        let tri0 = TriangleV4::new(self.point1, self.point2, self.point3);
        let tri1 = TriangleV4::new(self.point1, self.point0, self.point2);
        let tri2 = TriangleV4::new(self.point1, self.point3, self.point0);
        let tri3 = TriangleV4::new(self.point2, self.point0, self.point3);
        answer.push(tri0);
        answer.push(tri1);
        answer.push(tri2);
        answer.push(tri3);

        answer
    }
    fn new(
        point0: Vector4<f64>,
        point1: Vector4<f64>,
        point2: Vector4<f64>,
        point3: Vector4<f64>,
    ) -> PyramidV4 {
        let answer = PyramidV4 {
            point0: point0,
            point1: point1,
            point2: point2,
            point3: point3,
        };
        answer
    }
}
