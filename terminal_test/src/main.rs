use crossterm::style::Print;
use crossterm::{
    self, ExecutableCommand, QueueableCommand, cursor,
    event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, poll, read},
    execute,
    terminal::{
        self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};
use nalgebra::{self, Rotation3, UnitVector3, Vector3, Vector4};
use std::fmt;
use std::io::{self, Write, stdout};
use std::time::Duration;

fn main() -> io::Result<()> {
    let p0 = Vector4::new(0.0, 0.0, 0.0, 1.0);
    let p1 = Vector4::new(10.0, 0.0, 0.0, 1.0);
    let p2 = Vector4::new(0.0, 10.0, 0.0, 1.0);
    let p3 = Vector4::new(0.0, 0.0, 10.0, 1.0);
    let rotation_vector = Vector4::new(0.0, 0.0, 1.0, 0.0);

    let mut pyr0 = PyramidV4::new(p0, p1, p2, p3);
    println!("{pyr0}");
    pyr0.rotate_by_vec4_mut(rotation_vector, 10.0);
    println!("{pyr0}");

    std::thread::sleep(Duration::from_millis(20000));
    // declare stdout
    let mut stdout = stdout();

    // enter alternate screen and raw mode
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    stdout.queue(cursor::MoveTo(5, 5))?;
    stdout.queue(cursor::MoveTo(10, 5))?;
    stdout.queue(cursor::Hide)?;
    stdout.queue(Print("s"))?;
    stdout.flush()?;

    //  LOOP START

    //     exapmle of drawing
    'main_loop: loop {
        if poll(Duration::from_millis(100))? {
            match read()? {
                Event::Key(event) => {
                    if event
                        == (KeyEvent {
                            code: KeyCode::Char('q'),
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            state: KeyEventState::NONE,
                        })
                    {
                        break 'main_loop;
                    };
                }
                _ => println!("!"),
            }
        }
    }

    //  LOOP END

    // exit alternate screen and this is the end, supposedly )))
    //
    stdout.queue(cursor::Show)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    disable_raw_mode()?;

    execute!(io::stdout(), LeaveAlternateScreen)
}

struct TriangleV4 {
    point0: Vector4<f64>,
    point1: Vector4<f64>,
    point2: Vector4<f64>,
}

impl TriangleV4 {
    fn get_normal(&self) -> Vector4<f64> {
        let vec_a = Vector3::new(
            self.point1.x - self.point0.x,
            self.point1.y - self.point0.y,
            self.point1.z - self.point0.z,
        );
        let vec_b = Vector3::new(
            self.point2.x - self.point0.x,
            self.point2.y - self.point0.y,
            self.point2.z - self.point0.z,
        );
        let cross_product = vec_a.cross(&vec_b);
        let normalize_cross_prod = cross_product.normalize();
        let answer = Vector4::new(
            normalize_cross_prod.x,
            normalize_cross_prod.y,
            normalize_cross_prod.z,
            0.0,
        );
        answer
    }
    fn is_visible(&self, pov_vec4: Vector4<f64>) -> bool {
        let mut answer = false;

        let norm = self.get_normal();
        let angle = norm.angle(&pov_vec4);

        if angle < 90.0 {
            answer = true;
        }

        answer
    }
    fn get_points(&self) -> Vec<Vector4<f64>> {
        let mut answer = Vec::new();
        answer.push(self.point0);
        answer.push(self.point1);
        answer.push(self.point2);

        answer
    }
    fn new(p0: Vector4<f64>, p1: Vector4<f64>, p2: Vector4<f64>) -> TriangleV4 {
        let answer = TriangleV4 {
            point0: p0,
            point1: p1,
            point2: p2,
        };
        answer
    }
}

struct PyramidV4 {
    point0: Vector4<f64>,
    point1: Vector4<f64>,
    point2: Vector4<f64>,
    point3: Vector4<f64>,
}

impl PyramidV4 {
    fn get_points(&self) -> Vec<Vector4<f64>> {
        let mut answer = Vec::new();
        answer.push(self.point0);
        answer.push(self.point1);
        answer.push(self.point2);
        answer.push(self.point3);

        answer
    }
    fn get_triangles(&self) -> Vec<TriangleV4> {
        let mut answer = Vec::new();
        let tri0 = TriangleV4::new(self.point0, self.point1, self.point2);
        let tri1 = TriangleV4::new(self.point0, self.point2, self.point3);
        let tri2 = TriangleV4::new(self.point3, self.point1, self.point0);
        let tri3 = TriangleV4::new(self.point1, self.point3, self.point2);

        answer.push(tri0);
        answer.push(tri1);
        answer.push(tri2);
        answer.push(tri3);
        answer
    }
    fn rotate_by_vec4_mut(&mut self, rotation_vector: Vector4<f64>, angle_deg: f64) {
        let rotated_point0 = rot_by_vec4(self.point0, rotation_vector, angle_deg);
        let rotated_point1 = rot_by_vec4(self.point1, rotation_vector, angle_deg);
        let rotated_point2 = rot_by_vec4(self.point2, rotation_vector, angle_deg);
        let rotated_point3 = rot_by_vec4(self.point3, rotation_vector, angle_deg);
        self.point0 = rotated_point0;
        self.point1 = rotated_point1;
        self.point2 = rotated_point2;
        self.point3 = rotated_point3;
    }
    fn new(p0: Vector4<f64>, p1: Vector4<f64>, p2: Vector4<f64>, p3: Vector4<f64>) -> PyramidV4 {
        let temp = PyramidV4 {
            point0: p0,
            point1: p1,
            point2: p2,
            point3: p3,
        };
        temp
    }
}

fn rot_by_vec4(point: Vector4<f64>, rotation_vector: Vector4<f64>, angle_deg: f64) -> Vector4<f64> {
    let point_v3 = Vector3::new(point.x, point.y, point.z);
    let rot_vec_v3 = Vector3::new(rotation_vector.x, rotation_vector.y, rotation_vector.z);
    let unit = UnitVector3::new_normalize(rot_vec_v3);
    let rotation = Rotation3::from_axis_angle(&unit, angle_deg);
    let rotatiton_v3 = rotation * point_v3;
    let answer = Vector4::new(rotatiton_v3.x, rotatiton_v3.y, rotatiton_v3.z, 1.0);
    answer
}

impl fmt::Display for TriangleV4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p0x = self.point0.x;
        let p1x = self.point1.x;
        let p2x = self.point2.x;
        let p0y = self.point0.y;
        let p1y = self.point1.y;
        let p2y = self.point2.y;
        let p0z = self.point0.z;
        let p1z = self.point1.z;
        let p2z = self.point2.z;
        write!(
            f,
            "TriangleV4([{},{},{}],[{},{},{}],[{},{},{}])",
            p0x, p0y, p0z, p1x, p1y, p1z, p2x, p2y, p2z
        )
    }
}

impl fmt::Display for PyramidV4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let triangles = self.get_triangles();
        write!(
            f,
            "PyramidV4({}, {}, {})",
            triangles[0], triangles[1], triangles[2]
        )
    }
}
