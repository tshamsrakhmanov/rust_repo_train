use nalgebra::Vector4;

mod geometry_engine;
fn main() {
    let p1 = Vector4::new(10.0, 0.0, 0.0, 1.0);
    let p2 = Vector4::new(0.0, 10.0, 0.0, 1.0);
    let p3 = Vector4::new(0.0, 0.0, 10.0, 1.0);

    let tri = geometry_engine::Triangle::new(p1, p2, p3);
    println!("{tri}");
}
