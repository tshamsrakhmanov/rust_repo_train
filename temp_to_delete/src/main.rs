use std::{
    f64::consts::PI,
    io::{Write, stdout},
    time::Duration,
};

use bresenham::Bresenham as br;
use crossterm::{
    QueueableCommand, cursor,
    event::{Event, KeyCode, poll, read},
    execute,
    style::{self, Stylize},
    terminal::{
        self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use nalgebra::{Matrix4, Point3, Scale4, UnitVector3, Vector3, Vector4};
use std::cmp as comp;
use std::io;

fn main() -> io::Result<()> {
    let mut stdout = stdout();

    let screen_size = terminal::size()?;
    let dim_x = screen_size.0;
    let dim_y = screen_size.1;

    let left = -(dim_x as f64) / 2.0;
    let right = (dim_x as f64) / 2.0;
    let bottom = -(dim_y as f64) / 1.4;
    let top = (dim_y as f64) / 1.4;
    let znear = 0.0;
    let zfar = 100.0;

    let project_matrix = nalgebra::Matrix4::new_orthographic(left, right, bottom, top, znear, zfar);

    let view_x: f64 = 10.0;
    let view_y: f64 = 10.0;
    let view_z: f64 = 5.0;

    let visibility_vector = Vector4::new(view_x, view_y, view_z, 0.0);

    let eye = Point3::new(view_x, view_y, view_z);
    let target = Point3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 0.0, -1.0);

    let view_matrix = nalgebra::Matrix4::look_at_rh(&eye, &target, &up);

    let model_matrix: Matrix4<f64> = nalgebra::Matrix4::identity();

    let pvm = project_matrix * view_matrix * model_matrix;

    // Points declaration for PyramidV4
    // let point0 = Vector4::new(20.0, 0.0, -20.0, 1.0);
    // let point1 = Vector4::new(-20.0, 0.0, -20.0, 1.0);
    // let point2 = Vector4::new(0.0, -20.0, 20.0, 1.0);
    // let point3 = Vector4::new(0.0, 20.0, 20.0, 1.0);
    // let mut pyr0 = PyramidV4::new(point0, point1, point2, point3);

    let edge = 20.0;
    let point0 = Vector4::new(-edge, -edge, -edge, 1.0);
    let point1 = Vector4::new(-edge, edge, -edge, 1.0);
    let point2 = Vector4::new(edge, edge, -edge, 1.0);
    let point3 = Vector4::new(edge, -edge, -edge, 1.0);
    let point4 = Vector4::new(-edge, -edge, edge, 1.0);
    let point5 = Vector4::new(-edge, edge, edge, 1.0);
    let point6 = Vector4::new(edge, edge, edge, 1.0);
    let point7 = Vector4::new(edge, -edge, edge, 1.0);

    let mut c0 = CubeV4::new_by_points(
        point0, point1, point2, point3, point4, point5, point6, point7,
    );

    execute!(stdout, cursor::Hide)?;
    execute!(stdout, EnterAlternateScreen)?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    let mut screen_buffer: Vec<Pixel> = Vec::new();
    let mut rasterization_type = false;
    let mut pause = 50;

    let scale_vec_up = Vector4::new(1.2, 1.2, 1.2, 0.0);
    let scale_vec_down = Vector4::new(0.8, 0.8, 0.8, 0.0);
    let rotation_vec_x = Vector4::new(1.0, 0.0, 0.0, 0.0);
    let rotation_vec_y = Vector4::new(0.0, 1.0, 0.0, 0.0);
    let rotation_vec_z = Vector4::new(0.0, 0.0, 1.0, 0.0);
    let mut rotation_vec = rotation_vec_x;

    let mut rasterization_colored = true;
    enable_raw_mode()?;
    'main_loop: loop {
        // Randomized rotation vector
        // let mut rotation_vec = Vector4::new(
        //     rand::random_range(0.0..0.1),
        //     rand::random_range(0.0..0.1),
        //     rand::random_range(0.9..1.0),
        //     0.0,
        // );

        execute!(stdout, cursor::MoveTo(1, 1))?;
        execute!(stdout, style::PrintStyledContent("q - Exit".magenta()))?;
        execute!(stdout, cursor::MoveTo(1, 2))?;
        execute!(
            stdout,
            style::PrintStyledContent("w - change rasterization_type".magenta())
        )?;
        execute!(stdout, cursor::MoveTo(1, 3))?;
        execute!(
            stdout,
            style::PrintStyledContent("e - scale down model".magenta())
        )?;
        execute!(stdout, cursor::MoveTo(1, 4))?;
        execute!(
            stdout,
            style::PrintStyledContent("r - scale up model".magenta())
        )?;
        execute!(stdout, cursor::MoveTo(1, 5))?;
        execute!(
            stdout,
            style::PrintStyledContent("z - rotation by X-axis".magenta())
        )?;
        execute!(stdout, cursor::MoveTo(1, 6))?;
        execute!(
            stdout,
            style::PrintStyledContent("x - rotation by Y-axis".magenta())
        )?;
        execute!(stdout, cursor::MoveTo(1, 7))?;
        execute!(
            stdout,
            style::PrintStyledContent("c - rotation by Z-axis".magenta())
        )?;
        execute!(stdout, cursor::MoveTo(1, 8))?;
        execute!(stdout, style::PrintStyledContent("- - speed up".magenta()))?;
        execute!(stdout, cursor::MoveTo(1, 9))?;
        execute!(stdout, style::PrintStyledContent("= - slow down".magenta()))?;
        execute!(stdout, cursor::MoveTo(1, 10))?;
        execute!(
            stdout,
            style::PrintStyledContent("t - all white rasterization".magenta())
        )?;

        stdout.queue(Clear(ClearType::All))?;
        if poll(Duration::from_millis(pause))? {
            if let Event::Key(event) = read()? {
                if event.code == KeyCode::Char('q') {
                    break 'main_loop;
                }
                if event.code == KeyCode::Char('w') {
                    rasterization_type = !rasterization_type;
                }
                if event.code == KeyCode::Char('=') {
                    pause += 10;
                }
                if event.code == KeyCode::Char('-') {
                    pause -= 10;
                }
                if event.code == KeyCode::Char('e') {
                    c0.scale_mut(scale_vec_down);
                }
                if event.code == KeyCode::Char('r') {
                    c0.scale_mut(scale_vec_up);
                }
                if event.code == KeyCode::Char('z') {
                    rotation_vec = rotation_vec_x;
                }
                if event.code == KeyCode::Char('x') {
                    rotation_vec = rotation_vec_y;
                }
                if event.code == KeyCode::Char('c') {
                    rotation_vec = rotation_vec_z;
                }
                if event.code == KeyCode::Char('t') {
                    rasterization_colored = !rasterization_colored;
                }
            }
        }

        for pos in &screen_buffer {
            let x = pos.x;
            let y = pos.y;
            execute!(stdout, cursor::MoveTo(x, y))?;
            if rasterization_colored {
                if pos.color == 0 {
                    execute!(stdout, style::PrintStyledContent("█".magenta()))?;
                }
                if pos.color == 1 {
                    execute!(stdout, style::PrintStyledContent("█".yellow()))?;
                }
                if pos.color == 2 {
                    execute!(stdout, style::PrintStyledContent("█".red()))?;
                }
                if pos.color == 3 {
                    execute!(stdout, style::PrintStyledContent("█".blue()))?;
                }
                if pos.color == 4 {
                    execute!(stdout, style::PrintStyledContent("█".cyan()))?;
                }
            } else {
                execute!(stdout, style::PrintStyledContent("█".white()))?;
            }
        }

        stdout.flush()?;

        // pyr0.rotate_by_axis_mut(10.0, rotation_vec);
        c0.rotate_by_axis_mut(10.0, rotation_vec);

        screen_buffer.clear();
        // for pos in pyr0.get_triangles() {
        //     let visibility = pos.is_visible(&visibility_vector);
        //     if visibility {
        //         let tr = pos.project_to_screen(pvm, dim_x, dim_y);
        //
        //         if rasterization_type {
        //             for pos in tr.rasterize_to_fill() {
        //                 screen_buffer.push(pos);
        //             }
        //         } else {
        //             for pos in tr.resterize_to_lines() {
        //                 screen_buffer.push(pos);
        //             }
        //         }
        //     }
        // }
        for pos in c0.get_triangles() {
            let visibility = pos.is_visible(&visibility_vector);
            if visibility {
                let tr = pos.project_to_screen(pvm, dim_x, dim_y);

                if rasterization_type {
                    for pos in tr.rasterize_to_fill() {
                        screen_buffer.push(pos);
                    }
                } else {
                    for pos in tr.resterize_to_lines() {
                        screen_buffer.push(pos);
                    }
                }
            }
        }
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
    color: u16,
}

#[derive(Debug)]
struct TriangleV3 {
    point0: Vector3<f64>,
    point1: Vector3<f64>,
    point2: Vector3<f64>,
    color: u16,
}

impl TriangleV3 {
    fn new(
        point0: Vector3<f64>,
        point1: Vector3<f64>,
        point2: Vector3<f64>,
        color: u16,
    ) -> TriangleV3 {
        let answer = TriangleV3 {
            point0: point0,
            point1: point1,
            point2: point2,
            color: color,
        };
        answer
    }
    fn resterize_to_lines(&self) -> Vec<Pixel> {
        let mut answer: Vec<Pixel> = Vec::new();

        let p0 = (self.point0.x as isize, self.point0.y as isize);
        let p1 = (self.point1.x as isize, self.point1.y as isize);
        let p2 = (self.point2.x as isize, self.point2.y as isize);
        let l1 = br::new(p0, p1);
        let l2 = br::new(p1, p2);
        let l3 = br::new(p0, p2);

        for pos in l1 {
            answer.push(Pixel {
                x: pos.0 as u16,
                y: pos.1 as u16,
                color: self.color,
            });
        }

        for pos in l2 {
            answer.push(Pixel {
                x: pos.0 as u16,
                y: pos.1 as u16,
                color: self.color,
            });
        }

        for pos in l3 {
            answer.push(Pixel {
                x: pos.0 as u16,
                y: pos.1 as u16,
                color: self.color,
            });
        }
        answer
    }
    fn rasterize_to_fill(&self) -> Vec<Pixel> {
        let mut answer: Vec<Pixel> = Vec::new();
        let p0_x_u16 = self.point0.x as u16;
        let p1_x_u16 = self.point1.x as u16;
        let p2_x_u16 = self.point2.x as u16;
        let p0_y_u16 = self.point0.y as u16;
        let p1_y_u16 = self.point1.y as u16;
        let p2_y_u16 = self.point2.y as u16;
        let min_x = comp::min(comp::min(p0_x_u16, p1_x_u16), p2_x_u16);
        let max_x = comp::max(comp::max(p0_x_u16, p1_x_u16), p2_x_u16);
        let min_y = comp::min(comp::min(p0_y_u16, p1_y_u16), p2_y_u16);
        let max_y = comp::max(comp::max(p0_y_u16, p1_y_u16), p2_y_u16);

        let edge_0 = Vector3::new(
            self.point1.x - self.point0.x,
            self.point1.y - self.point0.y,
            self.point1.z - self.point0.z,
        );

        let edge_1 = Vector3::new(
            self.point2.x - self.point1.x,
            self.point2.y - self.point1.y,
            self.point2.z - self.point2.z,
        );

        let edge_2 = Vector3::new(
            self.point0.x - self.point2.x,
            self.point0.y - self.point2.y,
            self.point0.z - self.point2.z,
        );
        for x in min_x..max_x {
            for y in min_y..max_y {
                let x_f64 = x as f64;
                let y_f64 = y as f64;
                let temp_vec_0 = Vector3::new(x_f64 - self.point0.x, y_f64 - self.point0.y, 1.0);
                let temp_vec_1 = Vector3::new(x_f64 - self.point1.x, y_f64 - self.point1.y, 1.0);
                let temp_vec_2 = Vector3::new(x_f64 - self.point2.x, y_f64 - self.point2.y, 1.0);
                let w0 = edge_0.cross(&temp_vec_0);
                let w1 = edge_1.cross(&temp_vec_1);
                let w2 = edge_2.cross(&temp_vec_2);
                if w0.z > 0.0 && w1.z > 0.0 && w2.z > 0.0 {
                    answer.push(Pixel {
                        x: x,
                        y: y,
                        color: self.color,
                    });
                }
            }
        }

        answer
    }
}

impl TriangleV4 {
    fn new(
        point0: Vector4<f64>,
        point1: Vector4<f64>,
        point2: Vector4<f64>,
        color: u16,
    ) -> TriangleV4 {
        let a = TriangleV4 {
            point0: point0,
            point1: point1,
            point2: point2,
            color: color,
        };
        a
    }
    fn get_normal(&self) -> Vector4<f64> {
        let v1 = Vector3::new(
            self.point1.x - self.point0.x,
            self.point1.y - self.point0.y,
            self.point1.z - self.point0.z,
        );

        let v2 = Vector3::new(
            self.point2.x - self.point0.x,
            self.point2.y - self.point0.y,
            self.point2.z - self.point0.z,
        );
        let cross = v1.cross(&v2);
        let cross_v4 = Vector4::new(cross.x, cross.y, cross.z, 1.0);
        cross_v4
    }
    fn is_visible(&self, vector: &Vector4<f64>) -> bool {
        let mut answer = false;
        let normal = self.get_normal();
        let angle = normal.angle(vector);
        let angle_to_deg = angle * 180.0 / PI;
        if angle_to_deg < 90.0 {
            answer = true;
        }
        answer
    }
    fn project_to_screen(&self, pvm: Matrix4<f64>, dim_x: u16, dim_y: u16) -> TriangleV3 {
        let p0 = pvm * self.point0;
        let p1 = pvm * self.point1;
        let p2 = pvm * self.point2;

        let temp_x_0 = (dim_x as f64 / 2.0) * (1.0 + p0.x);
        let temp_x_1 = (dim_x as f64 / 2.0) * (1.0 + p1.x);
        let temp_x_2 = (dim_x as f64 / 2.0) * (1.0 + p2.x);

        let temp_y_0 = (dim_y as f64 / 2.0) * (1.0 + p0.y);
        let temp_y_1 = (dim_y as f64 / 2.0) * (1.0 + p1.y);
        let temp_y_2 = (dim_y as f64 / 2.0) * (1.0 + p2.y);

        let p0: Vector3<f64> = Vector3::new(temp_x_0, temp_y_0, 0.0);
        let p1: Vector3<f64> = Vector3::new(temp_x_1, temp_y_1, 0.0);
        let p2: Vector3<f64> = Vector3::new(temp_x_2, temp_y_2, 0.0);

        let a = TriangleV3 {
            point0: p0,
            point1: p1,
            point2: p2,
            color: self.color,
        };
        a
    }
}

impl PyramidV4 {
    fn get_triangles(&self) -> Vec<TriangleV4> {
        let mut answer: Vec<TriangleV4> = Vec::new();
        let tri0 = TriangleV4::new(self.point1, self.point2, self.point3, 0);
        let tri1 = TriangleV4::new(self.point1, self.point0, self.point2, 1);
        let tri2 = TriangleV4::new(self.point1, self.point3, self.point0, 2);
        let tri3 = TriangleV4::new(self.point2, self.point0, self.point3, 3);
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
    fn rotate_by_axis_mut(&mut self, degrees: f64, vector: Vector4<f64>) {
        self.point0 = rot_by_z(self.point0, vector, degrees);
        self.point1 = rot_by_z(self.point1, vector, degrees);
        self.point2 = rot_by_z(self.point2, vector, degrees);
        self.point3 = rot_by_z(self.point3, vector, degrees);
    }
}

fn rot_by_z(point: Vector4<f64>, vector: Vector4<f64>, degrees: f64) -> Vector4<f64> {
    let v3_vec = Vector3::new(vector.x, vector.y, vector.z);
    let v3_vec_norm = v3_vec.normalize();
    let unit = UnitVector3::from_ref_unchecked(&v3_vec_norm);
    let rotation_axis = Matrix4::from_axis_angle(&unit, deg_to_rad(degrees));
    let rot = rotation_axis * point;
    rot
}
fn scale_by_vec(point: Vector4<f64>, vector: &Vector4<f64>) -> Vector4<f64> {
    let scale = Scale4::new(vector.x, vector.y, vector.z, vector.w);
    let v = scale * point;
    v
}

fn deg_to_rad(deg: f64) -> f64 {
    let answer = deg * PI / 180.0;
    answer
}

struct Pixel {
    x: u16,
    y: u16,
    color: u16,
}

struct CubeV4 {
    point0: Vector4<f64>,
    point1: Vector4<f64>,
    point2: Vector4<f64>,
    point3: Vector4<f64>,
    point4: Vector4<f64>,
    point5: Vector4<f64>,
    point6: Vector4<f64>,
    point7: Vector4<f64>,
}

impl CubeV4 {
    fn new_by_points(
        point0: Vector4<f64>,
        point1: Vector4<f64>,
        point2: Vector4<f64>,
        point3: Vector4<f64>,
        point4: Vector4<f64>,
        point5: Vector4<f64>,
        point6: Vector4<f64>,
        point7: Vector4<f64>,
    ) -> CubeV4 {
        let c = CubeV4 {
            point0: point0,
            point1: point1,
            point2: point2,
            point3: point3,
            point4: point4,
            point5: point5,
            point6: point6,
            point7: point7,
        };
        c
    }
    fn get_triangles(&self) -> Vec<TriangleV4> {
        let mut answer: Vec<TriangleV4> = Vec::new();
        //bot
        let t0 = TriangleV4::new(self.point2, self.point0, self.point1, 0);
        let t1 = TriangleV4::new(self.point2, self.point3, self.point0, 0);

        //top
        let t2 = TriangleV4::new(self.point5, self.point4, self.point6, 1);
        let t3 = TriangleV4::new(self.point7, self.point6, self.point4, 1);

        //right
        let t4 = TriangleV4::new(self.point2, self.point1, self.point5, 2);
        let t5 = TriangleV4::new(self.point2, self.point5, self.point6, 2);

        //left
        let t6 = TriangleV4::new(self.point3, self.point7, self.point4, 3);
        let t7 = TriangleV4::new(self.point3, self.point4, self.point0, 3);

        //front
        let t8 = TriangleV4::new(self.point2, self.point6, self.point7, 0);
        let t9 = TriangleV4::new(self.point2, self.point7, self.point3, 0);

        //back
        let t10 = TriangleV4::new(self.point1, self.point4, self.point5, 1);
        let t11 = TriangleV4::new(self.point1, self.point0, self.point4, 1);

        answer.push(t0);
        answer.push(t1);

        answer.push(t2);
        answer.push(t3);

        answer.push(t4);
        answer.push(t5);

        answer.push(t6);
        answer.push(t7);

        answer.push(t8);
        answer.push(t9);

        answer.push(t10);
        answer.push(t11);

        answer
    }

    fn rotate_by_axis_mut(&mut self, degrees: f64, vector: Vector4<f64>) {
        self.point0 = rot_by_z(self.point0, vector, degrees);
        self.point1 = rot_by_z(self.point1, vector, degrees);
        self.point2 = rot_by_z(self.point2, vector, degrees);
        self.point3 = rot_by_z(self.point3, vector, degrees);
        self.point4 = rot_by_z(self.point4, vector, degrees);
        self.point5 = rot_by_z(self.point5, vector, degrees);
        self.point6 = rot_by_z(self.point6, vector, degrees);
        self.point7 = rot_by_z(self.point7, vector, degrees);
    }

    fn scale_mut(&mut self, vector: Vector4<f64>) {
        self.point0 = scale_by_vec(self.point0, &vector);
        self.point1 = scale_by_vec(self.point1, &vector);
        self.point2 = scale_by_vec(self.point2, &vector);
        self.point3 = scale_by_vec(self.point3, &vector);
        self.point4 = scale_by_vec(self.point4, &vector);
        self.point5 = scale_by_vec(self.point5, &vector);
        self.point6 = scale_by_vec(self.point6, &vector);
        self.point7 = scale_by_vec(self.point7, &vector);
    }
}
