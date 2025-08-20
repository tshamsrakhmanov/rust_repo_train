use nalgebra::Matrix4;
use nalgebra::Point3;
use nalgebra::Rotation3;
use nalgebra::Scale3;
use nalgebra::Translation3;
use nalgebra::Unit;
use nalgebra::Vector3;
use nalgebra::Vector4;
use std::f64::consts::PI;
use std::fmt::{self};

// to describe object in space - in coordinate system

pub struct Triangle {
    point0: Vector4<f64>,
    point1: Vector4<f64>,
    point2: Vector4<f64>,
}

impl Triangle {
    pub fn new(p0: Vector4<f64>, p1: Vector4<f64>, p2: Vector4<f64>) -> Triangle {
        let r = Triangle {
            point0: p0,
            point1: p1,
            point2: p2,
        };

        r
    }
    pub fn _get_points(&self) -> Vec<Vector4<f64>> {
        let mut r: Vec<Vector4<f64>> = Vec::new();
        r.push(self.point0);
        r.push(self.point1);
        r.push(self.point2);
        r
    }

    pub fn is_visible(&self, pov: Vector4<f64>) -> bool {
        let mut answer = false;
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
        let cross_prd = v1.cross(&v2);
        let temp_v4_to_v3 = Vector3::new(pov.x, pov.y, pov.z);
        let angle_rad = cross_prd.angle(&temp_v4_to_v3);
        let angle_deg = angle_rad.to_degrees();
        if angle_deg < 90.0 {
            answer = true;
        }
        // println!("{angle_deg}");
        answer
    }

    pub fn get_normal_vector(&self) -> Vector4<f64> {
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
        let cross_prd = v1.cross(&v2);
        let v4_to_v3 = Vector4::new(cross_prd.x, cross_prd.y, cross_prd.z, 0.0);
        let answer = v4_to_v3.normalize();
        answer
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p0x = self.point0.x.to_string();
        let p0y = self.point0.y.to_string();
        let p0z = self.point0.z.to_string();
        let p1x = self.point1.x.to_string();
        let p1y = self.point1.y.to_string();
        let p1z = self.point1.z.to_string();
        let p2x = self.point2.x.to_string();
        let p2y = self.point2.y.to_string();
        let p2z = self.point2.z.to_string();

        let p0: String = String::from(p0x + "," + &p0y + "," + &p0z);
        let p1: String = String::from(p1x + "," + &p1y + "," + &p1z);
        let p2: String = String::from(p2x + "," + &p2y + "," + &p2z);

        write!(f, "Triangle ({}, {}, {})", p0, p1, p2)
    }
}

impl fmt::Display for Pyramid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p0x = self.point0.x.to_string();
        let p0y = self.point0.y.to_string();
        let p0z = self.point0.z.to_string();
        let p1x = self.point1.x.to_string();
        let p1y = self.point1.y.to_string();
        let p1z = self.point1.z.to_string();
        let p2x = self.point2.x.to_string();
        let p2y = self.point2.y.to_string();
        let p2z = self.point2.z.to_string();
        let p3x = self.point3.x.to_string();
        let p3y = self.point3.y.to_string();
        let p3z = self.point3.z.to_string();

        let p0: String = String::from(p0x + "," + &p0y + "," + &p0z);
        let p1: String = String::from(p1x + "," + &p1y + "," + &p1z);
        let p2: String = String::from(p2x + "," + &p2y + "," + &p2z);
        let p3: String = String::from(p3x + "," + &p3y + "," + &p3z);

        write!(f, "Pyramid ({}, {}, {}, {})", p0, p1, p2, p3)
    }
}

pub struct Pyramid {
    point0: Vector4<f64>,
    point1: Vector4<f64>,
    point2: Vector4<f64>,
    point3: Vector4<f64>,
}

impl Pyramid {
    pub fn new(p1: Vector4<f64>, p2: Vector4<f64>, p3: Vector4<f64>, p4: Vector4<f64>) -> Pyramid {
        let r = Pyramid {
            point0: p1,
            point1: p2,
            point2: p3,
            point3: p4,
        };
        r
    }

    pub fn _get_points(&self) -> Vec<Vector4<f64>> {
        let mut r: Vec<Vector4<f64>> = Vec::new();
        r.push(self.point0);
        r.push(self.point1);
        r.push(self.point2);
        r.push(self.point3);
        r
    }

    pub fn get_triangles(&self) -> Vec<Triangle> {
        let mut r = Vec::new();
        let t0: Triangle = Triangle::new(self.point0, self.point1, self.point2);
        let t1: Triangle = Triangle::new(self.point1, self.point3, self.point2);
        let t2: Triangle = Triangle::new(self.point2, self.point3, self.point0);
        let t3: Triangle = Triangle::new(self.point1, self.point0, self.point3);
        r.push(t0);
        r.push(t1);
        r.push(t2);
        r.push(t3);
        r
    }

    pub fn _rotate_by_vector(point: &Vector4<f64>, angle_radians: f64) -> Vector4<f64> {
        let rotation = Rotation3::from_axis_angle(&Vector3::z_axis(), angle_radians);
        let rotation_matrix = Matrix4::from(rotation);

        rotation_matrix * point
    }
}

pub fn v4_rot(point: Vector4<f64>, vector: &Vector4<f64>, angle_radians: f64) -> Vector4<f64> {
    let rotation_vector_to_v3 = Vector3::new(vector.x, vector.y, vector.z);
    let rotation_vector_to_normilized_unit = Unit::new_normalize(rotation_vector_to_v3);
    let rotation = Rotation3::from_axis_angle(&rotation_vector_to_normilized_unit, angle_radians);
    let rotation_matrix = Matrix4::from(rotation);

    rotation_matrix * point
}

pub fn v4_trs(point: Vector4<f64>, vector: &Vector4<f64>) -> Vector4<f64> {
    let tranl_v3 = Translation3::new(vector.x, vector.y, vector.z);
    let point_v3 = Point3::new(point.x, point.y, point.z);
    let r = tranl_v3 * point_v3;
    let point_v4 = Vector4::new(r.x, r.y, r.z, 1.0);
    point_v4
}

pub fn v4_scl(point: Vector4<f64>, vector: &Vector4<f64>) -> Vector4<f64> {
    let scale_v3 = Scale3::new(vector.x, vector.y, vector.z);
    let point_v3 = Point3::new(point.x, point.y, point.z);
    let s = scale_v3 * point_v3;
    let point_v4 = Vector4::new(s.x, s.y, s.z, 1.0);
    point_v4
}

pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
