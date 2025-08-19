use nalgebra::Vector4;

mod geometry_engine;
fn main() {
    let p0 = Vector4::new(10.0, 0.0, 0.0, 1.0);
    let p1 = Vector4::new(0.0, 10.0, 0.0, 1.0);
    let p2 = Vector4::new(0.0, 0.0, 10.0, 1.0);
    let p3 = Vector4::new(0.0, 0.0, 0.0, 1.0);

    let pov_vec = Vector4::new(5.0, -5.0, 5.0, 0.0);

    let pyr = geometry_engine::Pyramid::new(p0, p1, p2, p3);
    let triangles_of_pyramid = pyr.get_triangles();
    for (i, pos) in triangles_of_pyramid.iter().enumerate() {
        let visibility = pos.is_visible(pov_vec);
        println!("   {i}-{visibility}");
    }
}
