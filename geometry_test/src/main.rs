use nalgebra::Vector4;

use crate::geometry_engine::Pyramid;

mod geometry_engine;
fn main() {
    let p1 = Vector4::new(10.0, 10.0, 10.0, 1.0);
    let p2 = Vector4::new(20.0, 20.0, 20.0, 1.0);
    let p3 = Vector4::new(30.0, 30.0, 30.0, 1.0);
    let p4 = Vector4::new(40.0, 40.0, 40.0, 1.0);
    let mut p = geometry_engine::Pyramid::new(p1, p2, p3, p4);
    let points = p.get_points();
    for pos in points {
        println!("{pos}");
    }
    p.rot_z();
    let points = p.get_points();
    for pos in points {
        println!("{pos}");
    }
    p = Pyramid::new(p1, p2, p3, p4);
}
