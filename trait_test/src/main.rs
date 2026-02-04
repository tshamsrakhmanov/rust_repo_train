use crate::structures::{Camera, Lambretian, Metal, Sphere, World};
use nalgebra::Vector3;
mod aux_fn;
mod structures;

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 800;
    let sample_per_pixel: i32 = 100;
    let jumps = 50;

    let cam1 = Camera::new(aspect_ratio, image_width, sample_per_pixel, jumps);

    let material_ground = Lambretian::new(Vector3::new(0.8, 0.8, 0.0));
    let material_center = Lambretian::new(Vector3::new(0.1, 0.2, 0.5));
    let material_left = Metal::new(Vector3::new(0.8, 0.8, 0.8), 20.0);
    let material_right = Metal::new(Vector3::new(0.8, 0.6, 0.2), 2.0);

    let ground_sphere = Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(material_ground),
    );
    let center_sphere = Sphere::new(Vector3::new(0.0, 0.0, -1.2), 0.5, Box::new(material_center));
    let left_sphere = Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, Box::new(material_left));
    let right_sphere = Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, Box::new(material_right));

    // world definition
    let mut w = World::new();
    w.add_object(Box::new(ground_sphere));
    w.add_object(Box::new(center_sphere));
    w.add_object(Box::new(left_sphere));
    w.add_object(Box::new(right_sphere));

    let _ = cam1.render(&w);
}
