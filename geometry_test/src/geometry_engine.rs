use nalgebra::Vector4;
use std::fmt;

pub struct Triangle {
    point1: Vector4<f64>,
    point2: Vector4<f64>,
    point3: Vector4<f64>,
}

impl Triangle {
    pub fn new(p1: Vector4<f64>, p2: Vector4<f64>, p3: Vector4<f64>) -> Triangle {
        let r = Triangle {
            point1: p1,
            point2: p2,
            point3: p3,
        };

        r
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.point1, self.point2, self.point3)
    }
}
