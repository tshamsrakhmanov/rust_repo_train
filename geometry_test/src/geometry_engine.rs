use nalgebra::Vector4;

pub struct Pyramid {
    point1: Vector4<f64>,
    point2: Vector4<f64>,
    point3: Vector4<f64>,
    point4: Vector4<f64>,
}

impl Pyramid {
    pub fn get_points(&self) -> Vec<Vector4<f64>> {
        let mut r = Vec::new();
        r.push(self.point1);
        r.push(self.point2);
        r.push(self.point3);
        r.push(self.point4);
        r
    }
    pub fn new(p1: Vector4<f64>, p2: Vector4<f64>, p3: Vector4<f64>, p4: Vector4<f64>) -> Pyramid {
        let r = Pyramid {
            point1: p1,
            point2: p2,
            point3: p3,
            point4: p4,
        };
        r
    }
    pub fn rot_z(&mut self) {
        self.point1.x = 1.0;
        self.point2.x = 1.0;
        self.point3.x = 1.0;
        self.point4.x = 1.0;
    }
}
