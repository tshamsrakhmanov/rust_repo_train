use nalgebra::Vector4;

use crate::geometry_engine::Pyramid;

mod geometry_engine;
fn main() {
    let p0 = Vector4::new(10.0, 0.0, 0.0, 1.0);
    let p1 = Vector4::new(0.0, 10.0, 0.0, 1.0);
    let p2 = Vector4::new(0.0, 0.0, 10.0, 1.0);
    let p3 = Vector4::new(0.0, 0.0, 0.0, 1.0);

    let pov_vec = Vector4::new(-5.0, -5.0, -5.0, 0.0);

    let pyr1 = geometry_engine::Pyramid::new(p0, p1, p2, p3);

    let mut scene: Vec<Pyramid> = Vec::new();
    scene.push(pyr1);

    // visibility check
    println!("Objects in area:");
    for pos in scene {
        println!(" {pos}");
        let triangles = pos.get_triangles();
        println!("Visible triangle of it:");
        for triangle in triangles {
            let visibility = triangle.is_visible(pov_vec);
            if visibility {
                println!(" {triangle}");
            }
        }
    }
}
