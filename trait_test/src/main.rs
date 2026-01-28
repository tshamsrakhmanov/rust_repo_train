use nalgebra::Vector3;

use crate::structures::{Camera, Sphere, World};
mod aux_fn;
mod structures;

fn main() {
    let ar_new: f32 = 16.0 / 9.0;
    let iw_new = 400;
    let mut cam1 = Camera::new(ar_new, iw_new);

    let p3 = Vector3::new(0.0, 0.0, -1.0);
    let r3: f32 = 0.5;
    let p4 = Vector3::new(0.0, -100.5, -1.0);
    let r4: f32 = 100.0;
    let s3 = Sphere::new(p3, r3);
    let s4 = Sphere::new(p4, r4);

    // world definition
    let mut world2 = World::new();
    world2.add_object(s3);
    world2.add_object(s4);

    let _ = cam1.render(&world2);
}
