use nalgebra::Vector4;

use crate::geometry_engine::{Pyramid, deg_to_rad, v4_rot, v4_scl, v4_trs};

mod geometry_engine;
fn main() {
    let p0 = Vector4::new(10.0, 0.0, 0.0, 1.0);
    let p1 = Vector4::new(0.0, 10.0, 0.0, 1.0);
    let p2 = Vector4::new(0.0, 0.0, 10.0, 1.0);
    let p3 = Vector4::new(0.0, 0.0, 0.0, 1.0);

    let pov_vec = Vector4::new(5.0, 5.0, 5.0, 0.0);

    let pyr1 = geometry_engine::Pyramid::new(p0, p1, p2, p3);

    let mut scene: Vec<Pyramid> = Vec::new();
    scene.push(pyr1);

    // print view vector
    println!("-------------------");
    println!("Point of view vector:");
    println!(" {pov_vec:?}");

    // visibility check
    println!("Objects in area:");
    for object in scene {
        println!(" {object}");
        let triangles = object.get_triangles();
        println!("Visible triangle of it:");
        for triangle in triangles {
            let visibility = triangle.is_visible(pov_vec);
            if visibility {
                println!(" {triangle}");
                let normal_vec = triangle.get_normal_vector();
                println!(" n:{normal_vec:?}");
            }
        }
    }
    println!("-----------");
    println!("Before rotation:");
    let p4 = Vector4::new(10.0, 0.0, 0.0, 1.0);
    println!(" {p4:?}");
    let v1 = Vector4::new(0.0, 0.0, 1.0, 0.0);
    println!("Rotation vector:");
    println!("  {v1:?}");
    let p4_rot = v4_rot(p4, &v1, deg_to_rad(30.0));
    println!("After rotation:");
    println!("1: {p4_rot:?}");
    let p4_rot_1 = v4_rot(p4_rot, &v1, deg_to_rad(30.0));
    println!("2: {p4_rot_1:?}");
    let p4_rot_2 = v4_rot(p4_rot_1, &v1, deg_to_rad(30.0));
    println!("3: {p4_rot_2:?}");
    println!("--------------------");
    let p5 = Vector4::new(1.0, 1.0, 1.0, 1.0);
    println!("Point before translation:");
    println!(" {p5:?}");
    let v2 = Vector4::new(-1.0, -1.0, -1.0, 0.0);
    let p6 = v4_trs(p5, &v2);
    println!("Translation vector:");
    println!(" {v2:?}");
    println!("Point after translation:");
    println!(" {p6:?}");
    println!("-------------------");
    let p6 = Vector4::new(2.0, 2.0, 2.0, 1.0);
    println!("Point before scale:");
    println!(" {p6:?}");
    let v3 = Vector4::new(-2.0, 2.0, 2.0, 0.0);
    println!("Scale vector:");
    println!(" {v3:?}");
    let p7 = v4_scl(p6, &v3);
    println!("Point after scale:");
    println!(" {p7:?}");

    println!("-------------------");
}
