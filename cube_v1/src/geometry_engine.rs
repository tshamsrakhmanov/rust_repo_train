use nalgebra::Vector4;

pub struct Pyrammid {
    point1: Vector4<f64>,
    point2: Vector4<f64>,
    point3: Vector4<f64>,
    point4: Vector4<f64>,
}

impl Pyrammid {
    fn get_points(&self) -> Vec<Vector4<f64>> {
        let mut r: Vec<Vector4<f64>> = Vec::new();
        r.push(self.point1);
        r
    }
}
