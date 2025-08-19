use nalgebra::Vector3;
use nalgebra::Vector4;
use std::fmt::{self, write};

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
    pub fn get_points(&self) -> Vec<Vector4<f64>> {
        let mut r: Vec<Vector4<f64>> = Vec::new();
        r.push(self.point0);
        r.push(self.point1);
        r.push(self.point2);
        r
    }

    pub fn is_visible(&self, pov: Vector4<f64>) -> bool {
        let mut answer = false;
        let v1 = Vector3::new(self.point0.x, self.point0.y, self.point0.z);
        let v2 = Vector3::new(self.point1.x, self.point1.y, self.point1.z);
        let v1_cross_v2 = v1.cross(&v2);
        let v3 = Vector4::new(v1_cross_v2.x, v1_cross_v2.y, v1_cross_v2.z, 0.0);
        let angle_rad = v3.angle(&pov);
        let angle_deg = angle_rad.to_degrees();
        if angle_deg < 90.0 {
            answer = true;
        }
        answer
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.point0, self.point1, self.point2)
    }
}

impl fmt::Display for Pyramid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.point0, self.point1, self.point2, self.point3
        )
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

    pub fn get_points(&self) -> Vec<Vector4<f64>> {
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
        let t1: Triangle = Triangle::new(self.point2, self.point1, self.point3);
        let t2: Triangle = Triangle::new(self.point0, self.point2, self.point3);
        let t3: Triangle = Triangle::new(self.point1, self.point0, self.point3);
        r.push(t0);
        r.push(t1);
        r.push(t2);
        r.push(t3);
        r
    }
}
