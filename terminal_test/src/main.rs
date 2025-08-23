use crossterm::style::{Print, Stylize};
use crossterm::{
    self, ExecutableCommand, QueueableCommand, cursor,
    event::{Event, KeyCode, poll, read},
    execute,
    terminal::{
        self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode, size,
    },
};
use nalgebra::{self, Matrix4, Point3, Rotation3, UnitVector3, Vector3, Vector4, min};
use rand;
use round_float::RoundToFraction;
use std::cmp;
use std::f64::consts::PI;
use std::fmt;
use std::io::{self, Write, stdout};

fn main() -> io::Result<()> {
    // get window size

    let terminal_size = size()?;
    let dim_x = terminal_size.0 as f64;
    let dim_y = terminal_size.1 as f64;

    // preparation of most difficult part - projection matrix
    // in this exapmple ill not go so far and will use prepared info - such as some vectors and
    // points values

    let left = -dim_x / 2.0;
    let right = dim_x / 2.0;
    let bottom = -dim_y / 1.0;
    let top = dim_y / 1.0;
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
    let edge = 24.0;
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
    let angle_deg = 2.5;
    let mut screen_buffer: Vec<(u16, u16, u16)> = Vec::new();
    let pov_vec4 = Vector4::new(pov_x, pov_y, pov_z, 0.0);

    'main_loop: loop {
        if poll(std::time::Duration::from_millis(10))? {
            if let Event::Key(key_event) = read()? {
                if key_event.code == KeyCode::Char('q') {
                    break 'main_loop;
                }
            }
        }

        screen_buffer.clear();
        let rotation_vector = Vector4::new(
            rand::random_range(0.0..0.1),
            rand::random_range(0.0..0.1),
            rand::random_range(0.0..1.0),
            0.0,
        );
        // variable to store basic x-axis vector for rotation
        // let rotation_vector1 = Vector4::new(0.1, 0.1, 0.1, 0.0);

        pyramid.rotate_by_vec4_mut(rotation_vector, angle_deg);

        for triangle in pyramid.get_triangles() {
            if triangle.is_visible(&pov_vec4) {
                let p0_raw = triangle.point0;
                let p1_raw = triangle.point1;
                let p2_raw = triangle.point2;
                let prj_0 = projection(terminal_size.0, terminal_size.1, &pvm_matrix, &p0_raw);
                let prj_1 = projection(terminal_size.0, terminal_size.1, &pvm_matrix, &p1_raw);
                let prj_2 = projection(terminal_size.0, terminal_size.1, &pvm_matrix, &p2_raw);

                let tr1 = TriangleV2::new(
                    (prj_0.0 as i16, prj_0.1 as i16),
                    (prj_1.0 as i16, prj_1.1 as i16),
                    (prj_2.0 as i16, prj_2.1 as i16),
                    triangle.color,
                );

                let points_from_triangle = tr1.get_drawing_points();
                for p in points_from_triangle {
                    screen_buffer.push(p);
                }
            }
        }

        stdout.queue(Clear(ClearType::All))?;
        for pixel in &screen_buffer {
            stdout.queue(cursor::MoveTo(pixel.0, pixel.1))?;
            if pixel.2 == 1 {
                stdout.queue(crossterm::style::PrintStyledContent("▓".red()))?;
            }
            if pixel.2 == 2 {
                stdout.queue(crossterm::style::PrintStyledContent("▓".green()))?;
            }
            if pixel.2 == 3 {
                stdout.queue(crossterm::style::PrintStyledContent("▓".blue()))?;
            }
            if pixel.2 == 0 {
                stdout.queue(crossterm::style::PrintStyledContent("▓".white()))?;
            }
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
    color: u16,
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
    fn new(p0: Vector4<f64>, p1: Vector4<f64>, p2: Vector4<f64>, color: u16) -> TriangleV4 {
        let answer = TriangleV4 {
            point0: p0,
            point1: p1,
            point2: p2,
            color: color,
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
        let tri0 = TriangleV4::new(self.point0, self.point1, self.point2, 0);
        let tri1 = TriangleV4::new(self.point3, self.point1, self.point0, 1);
        let tri2 = TriangleV4::new(self.point2, self.point3, self.point0, 2);
        let tri3 = TriangleV4::new(self.point2, self.point1, self.point3, 3);

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

impl fmt::Display for TriangleV2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TriangleV2({} {},{} {},{} {})",
            self.point0.0,
            self.point0.1,
            self.point1.0,
            self.point1.1,
            self.point2.0,
            self.point2.1
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

struct TriangleV2 {
    point0: (i16, i16),
    point1: (i16, i16),
    point2: (i16, i16),
    color: u16,
}

impl TriangleV2 {
    fn new(point0: (i16, i16), point1: (i16, i16), point2: (i16, i16), color: u16) -> TriangleV2 {
        let a = TriangleV2 {
            point0: point0,
            point1: point1,
            point2: point2,
            color: color,
        };
        a
    }
    fn get_drawing_points(&self) -> Vec<(u16, u16, u16)> {
        let mut answer: Vec<(u16, u16, u16)> = Vec::new();
        let min_x = cmp::min(cmp::min(self.point0.0, self.point1.0), self.point2.0);
        let max_x = cmp::max(cmp::max(self.point0.0, self.point1.0), self.point2.0);
        let min_y = cmp::min(cmp::min(self.point0.1, self.point1.1), self.point2.1);
        let max_y = cmp::max(cmp::max(self.point0.1, self.point1.1), self.point2.1);
        let edge0_v3 = Vector3::new(
            (self.point1.0 - self.point0.0) as f64,
            (self.point1.1 - self.point0.1) as f64,
            0.0,
        );
        let edge1_v3 = Vector3::new(
            (self.point2.0 - self.point1.0) as f64,
            (self.point2.1 - self.point1.1) as f64,
            0.0,
        );
        let edge2_v3 = Vector3::new(
            (self.point0.0 - self.point2.0) as f64,
            (self.point0.1 - self.point2.1) as f64,
            0.0,
        );
        let top_left_rule_parameter_0 = is_top_left(edge0_v3, edge1_v3);
        let top_left_rule_parameter_1 = is_top_left(edge1_v3, edge2_v3);
        let top_left_rule_parameter_2 = is_top_left(edge2_v3, edge0_v3);
        let mut bias_0 = 0.0;
        let mut bias_1 = 0.0;
        let mut bias_2 = 0.0;
        if top_left_rule_parameter_0 == false {
            bias_0 = -1.0;
        }
        if top_left_rule_parameter_1 == false {
            bias_1 = -1.0;
        }
        if top_left_rule_parameter_2 == false {
            bias_2 = -1.0;
        }
        for x in min_x..max_x + 1 {
            for y in min_y..max_y + 1 {
                let p_coor_0 =
                    Vector3::new((x - self.point0.0) as f64, (y - self.point0.1) as f64, 0.0);
                let p_coor_1 =
                    Vector3::new((x - self.point1.0) as f64, (y - self.point1.1) as f64, 0.0);
                let p_coor_2 =
                    Vector3::new((x - self.point2.0) as f64, (y - self.point2.1) as f64, 0.0);
                let w0_raw = p_coor_0.cross(&edge0_v3).normalize();
                let w1_raw = p_coor_1.cross(&edge1_v3).normalize();
                let w2_raw = p_coor_2.cross(&edge2_v3).normalize();
                if w2_raw.z + bias_0 > 0.0 && w1_raw.z + bias_1 > 0.0 && w0_raw.z + bias_2 > 0.0 {
                    answer.push((x as u16, y as u16, self.color));
                }
            }
        }

        answer
    }
}
fn is_top_left(vector0: Vector3<f64>, vector1: Vector3<f64>) -> bool {
    let edge = Vector3::new(
        (vector1.x - vector0.x) as f64,
        (vector1.y - vector0.y) as f64,
        0.0,
    );
    let mut a1 = true;
    if edge.y == 0.0 && edge.x < 0.0 {
        a1 = false;
    }
    let mut a2 = true;
    if edge.y < 0.0 {
        a2 = false;
    }
    a1 || a2
}
