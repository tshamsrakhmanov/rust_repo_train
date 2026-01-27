use nalgebra::Vector3;
use std::fs::File;
use std::io::prelude::*;

use crate::{
    aux_fn::ray_color,
    structures::{Ray, Sphere, World},
};
mod aux_fn;
mod structures;

fn main() -> std::io::Result<()> {
    // screen parameters
    let image_width = 500;
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_height: i32 = ((image_width as f32) / aspect_ratio) as i32;

    // calculation of view_port and camera
    let focal_length: f32 = 1.0;
    let camera_center = Vector3::new(0.0, 0.0, 0.0);
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = viewport_height * (image_width as f32 / image_height as f32);

    // calculatie viewport vectors
    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    // calculate delta between pixels
    let pixel_delta_u = viewport_u / (image_width as f32);
    let pixel_delta_v = viewport_v / (image_height as f32);

    // calculate location of upper left pixel
    let viewport_upper_left =
        camera_center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // open file
    let mut file = File::create("pic.ppm")?;

    // write boilerplate of file type e.t.c...
    write!(file, "{}\n", "P3")?;
    write!(file, "{} {}\n", image_width, image_height)?;
    write!(file, "{}\n", 255)?;

    let p1 = Vector3::new(0.0, 0.0, -1.0);
    let r1: f32 = 0.5;
    let p2 = Vector3::new(0.0, -100.5, -1.0);
    let r2: f32 = 100.0;
    let s1 = Sphere::new(p1, r1);
    let s2 = Sphere::new(p2, r2);

    // world definition
    let mut world1 = World::new();
    world1.add_object(s1);
    world1.add_object(s2);

    // fill the space
    for y_pos in 0..image_height {
        let a = image_height - y_pos;
        println!("Scan lines remaining: {}", a);
        for x_pos in 0..image_width {
            let pixel_center =
                pixel00loc + (x_pos as f32 * pixel_delta_u) + (y_pos as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray1 = Ray::new(camera_center, ray_direction);
            let color = ray_color(&ray1, &world1);

            aux_fn::write_pixel(&mut file, color);
        }
    }
    Ok(())
}
