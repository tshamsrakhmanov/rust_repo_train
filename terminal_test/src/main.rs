use crossterm::style::Print;
use crossterm::{
    self, ExecutableCommand, QueueableCommand, cursor,
    event::{Event, KeyCode, poll, read},
    execute,
    terminal::{
        self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode, size,
    },
};
use nalgebra::{self, Matrix4, Point3, Rotation3, UnitVector3, Vector3, Vector4};
use rand;
use round_float::RoundToFraction;
use std::f64::consts::PI;
use std::fmt;
use std::io::{self, Write, stdout};

fn main() -> io::Result<()> {
    // get window size

    let terminal_size = size()?;
    println!("{terminal_size:?}");
    let dim_x = terminal_size.0 as f64;
    let dim_y = terminal_size.1 as f64;

    // preparation of most difficult part - projection matrix
    // in this exapmple ill not go so far and will use prepared info - such as some vectors and
    // points values

    let left = -dim_x / 2.0;
    let right = dim_x / 2.0;
    let bottom = -dim_y / 2.0;
    let top = dim_y / 2.0;
    let znear = 0.0;
    let zfar = 1.0;

    let pov_x = 10.0;
    let pov_y = 10.0;
    let pov_z = 0.0;
    let eye = Point3::new(pov_x, pov_y, pov_z);
    let target = Point3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 0.0, -1.0);

    let projection_matrix = Matrix4::new_orthographic(left, right, bottom, top, znear, zfar);
    let view_matrix = Matrix4::look_at_rh(&eye, &target, &up);
    let model1 = Matrix4::identity();
    // let model1: Matrix4<f64> = Matrix4::new(
    //     1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.8, 0.0, 0.0, 0.0, 0.0, 1.0,
    // );
    let pvm_matrix = projection_matrix * view_matrix * model1;

    // preparation of pyramid model
    let edge = 14.0;
    let p0 = Vector4::new(0.0, edge, edge, 1.0);
    let p1 = Vector4::new(0.0, -edge, edge, 1.0);
    let p2 = Vector4::new(edge, 0.0, -edge, 1.0);
    let p3 = Vector4::new(-edge, 0.0, -edge, 1.0);
    // let p0 = Vector4::new(edge, 0.0, 0.0, 1.0);
    // let p1 = Vector4::new(0.0, edge, 0.0, 1.0);
    // let p2 = Vector4::new(0.0, 0.0, edge, 1.0);
    // let p3 = Vector4::new(0.0, 0.0, 0.0, 1.0);
    let mut pyramid = PyramidV4::new(p0, p1, p2, p3);

    // declare stdout
    let mut stdout = stdout();

    // enter alternate screen and raw mode
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.queue(cursor::Hide)?;

    //  LOOP START

    // let rotation_vector = Vector4::new(0.2, 0.2, 0.2, 0.0);
    let angle_deg = 8.55;
    let mut screen_buffer: Vec<(u16, u16)> = Vec::new();

    'main_loop: loop {
        if poll(std::time::Duration::from_millis(20))? {
            if let Event::Key(key_event) = read()? {
                if key_event.code == KeyCode::Char('q') {
                    break 'main_loop;
                }
            }
        }

        screen_buffer.clear();
        let rotation_vector = Vector4::new(
            rand::random_range(0.0..0.3),
            rand::random_range(0.0..0.3),
            rand::random_range(0.0..1.0),
            0.0,
        );
        // let rotation_vector = Vector4::new(0.0, 0.0, 1.0, 0.0);
        pyramid.rotate_by_vec4_mut(rotation_vector, angle_deg);

        for trianle in pyramid.get_triangles() {
            let pov_vec4 = Vector4::new(pov_x, pov_y, pov_z, 0.0);
            if trianle.is_visible(&pov_vec4) {
                let p0_raw = trianle.point0;
                let p1_raw = trianle.point1;
                let p2_raw = trianle.point2;
                let prj_0 = projection(terminal_size.0, terminal_size.1, &pvm_matrix, &p0_raw);
                let prj_1 = projection(terminal_size.0, terminal_size.1, &pvm_matrix, &p1_raw);
                let prj_2 = projection(terminal_size.0, terminal_size.1, &pvm_matrix, &p2_raw);

                let br0 = bresenham::Bresenham::new(
                    (prj_0.0 as isize, prj_0.1 as isize),
                    (prj_1.0 as isize, prj_1.1 as isize),
                );
                let br1 = bresenham::Bresenham::new(
                    (prj_1.0 as isize, prj_1.1 as isize),
                    (prj_2.0 as isize, prj_2.1 as isize),
                );
                let br2 = bresenham::Bresenham::new(
                    (prj_2.0 as isize, prj_2.1 as isize),
                    (prj_0.0 as isize, prj_0.1 as isize),
                );

                for pos in br0 {
                    let x = pos.0 as u16;
                    let y = pos.1 as u16;
                    screen_buffer.push((x, y));
                }
                for pos in br1 {
                    let x = pos.0 as u16;
                    let y = pos.1 as u16;
                    screen_buffer.push((x, y));
                }
                for pos in br2 {
                    let x = pos.0 as u16;
                    let y = pos.1 as u16;
                    screen_buffer.push((x, y));
                }
                // for point in trianle.get_points() {
                //     let prj = projection(terminal_size.0, terminal_size.1, &pvm_matrix, &point);
                //
                //     screen_buffer.push(prj);
                // }
            }
        }

        stdout.queue(Clear(ClearType::All))?;
        for pixel in &screen_buffer {
            stdout.queue(cursor::MoveTo(pixel.0, pixel.1))?;
            stdout.queue(Print("█"))?;
        }
        stdout.flush()?;
    }
    //  LOOP END

    // exit alternate screen and this is the end, supposedly )))
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
    fn is_visible(&self, pov_vec4: &Vector4<f64>) -> bool {
        let mut answer = false;

        let norm = self.get_normal();
        let angle = norm.angle(&pov_vec4);
        let angle_deg = angle.to_degrees();

        if angle_deg < 90.0 {
            answer = true;
        }

        answer
    }
    fn _get_points(&self) -> Vec<Vector4<f64>> {
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
    fn _get_points(&self) -> Vec<Vector4<f64>> {
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
        let tri1 = TriangleV4::new(self.point3, self.point1, self.point0);
        let tri2 = TriangleV4::new(self.point2, self.point3, self.point0);
        let tri3 = TriangleV4::new(self.point2, self.point1, self.point3);

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

fn rot_by_vec4(point: Vector4<f64>, rotation_vector: Vector4<f64>, angle_rad: f64) -> Vector4<f64> {
    let rotation_vector_v3 = Vector3::new(rotation_vector.x, rotation_vector.y, rotation_vector.z);
    let normalized_rotation_vector_v3 = rotation_vector_v3.normalize();
    let unit_v3 = UnitVector3::new_normalize(normalized_rotation_vector_v3);
    let rotation_matrix_v3 = Rotation3::from_axis_angle(&unit_v3, deg_to_rad(angle_rad));
    let rotation_matrix_v4 = Matrix4::from(rotation_matrix_v3);

    rotation_matrix_v4 * point
}

fn deg_to_rad(deg: f64) -> f64 {
    let answer = (PI * deg) / 180.0;
    answer
}

impl fmt::Display for TriangleV4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p0x = self.point0.x.round_to_fraction(2);
        let p1x = self.point1.x.round_to_fraction(2);
        let p2x = self.point2.x.round_to_fraction(2);
        let p0y = self.point0.y.round_to_fraction(2);
        let p1y = self.point1.y.round_to_fraction(2);
        let p2y = self.point2.y.round_to_fraction(2);
        let p0z = self.point0.z.round_to_fraction(2);
        let p1z = self.point1.z.round_to_fraction(2);
        let p2z = self.point2.z.round_to_fraction(2);
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
            "PyramidV4({}\n          {}\n          {})",
            triangles[0], triangles[1], triangles[2]
        )
    }
}

fn projection(
    dim_x: u16,
    dim_y: u16,
    pvm_matrix: &Matrix4<f64>,
    point: &Vector4<f64>,
) -> (u16, u16) {
    let projected_point = pvm_matrix * point;
    let point_wo_w = projected_point / projected_point.w;

    // calculation of x goes with subtraction fro dim-x because of screen reflection
    // just beleive me for now - it'll work in future
    let x_raw = dim_x as f64 - (dim_x as f64 / 2.0) * (1.0 + point_wo_w.x);
    let y_raw = (dim_y as f64 / 2.0) * (1.0 + point_wo_w.y);

    (x_raw as u16, y_raw as u16)
}
