use crate::structures::{Camera, Lambretian, Metal, Sphere, World};
use nalgebra::Vector3;
mod aux_fn;
mod structures;

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 1920;
    let sample_per_pixel: i32 = 200;
    let jumps = 50;

    let cam1 = Camera::new(aspect_ratio, image_width, sample_per_pixel, jumps);

    let coor_small_sphere = Vector3::new(0.0, 0.0, -1.0);
    let radius_small_sphere: f32 = 0.5;
    let coor_big_sphere = Vector3::new(0.0, -100.5, -1.0);
    let radius_big_sphere: f32 = 100.0;
    let material_small_sphere = Metal::new(Vector3::new(0.4, 0.4, 0.4));
    let material_big_sphere = Metal::new(Vector3::new(0.4, 0.4, 0.4));
    let small_sphere = Sphere::new(
        coor_small_sphere,
        radius_small_sphere,
        material_small_sphere,
    );
    let big_sphere = Sphere::new(coor_big_sphere, radius_big_sphere, material_big_sphere);

    // world definition
    let mut world2 = World::new();
    world2.add_object(small_sphere);
    world2.add_object(big_sphere);

    let _ = cam1.render(&world2);
}
