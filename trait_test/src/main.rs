use crate::structures::{Camera, Dielectric, Lambretian, Metal, Sphere, World};
use nalgebra::Vector3;
mod aux_fn;
mod structures;

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel: i32 = 100;
    let max_depth = 50;

    let lookfrom = Vector3::new(13.0, 2.0, 3.0);
    let lookat = Vector3::new(0.0, 0.0, 0.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.6;
    let focus_dist = 10.0;
    let vfov = 20.0;

    let cam1 = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        lookfrom,
        lookat,
        vfov,
        vup,
        defocus_angle,
        focus_dist,
    );

    let ground_mat = Box::new(Lambretian::new(Vector3::new(0.5, 0.5, 0.5)));
    let ground = Box::new(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    ));
    let m1 = Box::new(Dielectric::new(1.5));
    let s1 = Box::new(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, m1));
    let m2 = Box::new(Lambretian::new(Vector3::new(0.4, 0.2, 0.1)));
    let s2 = Box::new(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, m2));
    let m3 = Box::new(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0));
    let s3 = Box::new(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, m3));

    let mut w = World::new();
    w.add_object(ground);
    w.add_object(s1);
    w.add_object(s2);
    w.add_object(s3);

    let _ = cam1.render(&w);
}
