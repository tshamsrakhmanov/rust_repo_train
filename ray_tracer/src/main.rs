use std::fs::File;
use std::io::prelude::*;

use nalgebra::Vector3;

use crate::ray::ray_module::Ray;

mod ray;

fn main() -> std::io::Result<()> {
    // var declaration
    let image_width = 1920;
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_height: i32 = ((image_width as f32) / aspect_ratio) as i32;

    // calculation of view_port and camera
    let focal_length = 1.0;
    let camera_center = Vector3::new(0.0, 0.0, 0.0);
    let viewport_height: f32 = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

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

    // file connect
    let mut file = File::create("pic.ppm")?;

    // write boilerplate of file type e.t.c...
    write!(file, "{}\n", "P3")?;
    write!(file, "{} {}\n", image_width, image_height)?;
    write!(file, "{}\n", 255)?;

    // fill the space
    for y_pos in 0..image_height {
        let a = image_height - y_pos;
        println!("Scan lines remaining: {}", a);
        for x_pos in 0..image_width {
            let pixel_center =
                pixel00loc + (x_pos as f32 * pixel_delta_u) + (y_pos as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r1 = ray::ray_module::Ray::new(camera_center, ray_direction);
            let color: Vector3<f32> = ray_color(&r1);

            write_pixel(&mut file, color);
        }
    }
    println!("Done!");

    println!("{} {} {}", image_width, image_height, aspect_ratio);
    println!(
        "{} {} {}",
        viewport_width,
        viewport_height,
        viewport_width / viewport_height
    );

    println!("{}", pixel00loc);
    println!("{}", pixel00loc + image_width as f32 * pixel_delta_u);
    println!("{}", pixel00loc + image_height as f32 * pixel_delta_v);
    println!(
        "{}",
        pixel00loc + image_height as f32 * pixel_delta_v + image_width as f32 * pixel_delta_u
    );
    Ok(())
}

fn write_pixel(file: &mut File, pixel: Vector3<f32>) {
    let ir = (255.999 * pixel.x) as u8;
    let ig = (255.999 * pixel.y) as u8;
    let ib = (255.999 * pixel.z) as u8;

    let str1 = format!("{} {} {}\n", ir, ig, ib);
    let byt1 = str1.as_bytes();
    let _ = file.write(byt1);
}

fn ray_color(ray: &Ray) -> Vector3<f32> {
    // calculate sphere intersection with normales
    let t = hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let a: Vector3<f32> = (ray.at(t) - Vector3::new(0.0, 0.0, -1.0)).normalize();
        return 0.5 * Vector3::new(a.x + 1.0, a.y + 1.0, a.z + 1.0);
    }

    // calculate background color
    let unit_dicrection = ray.get_direction().normalize();
    let a = 0.5 * (unit_dicrection.y + 1.0);
    let bg_color = (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0);
    bg_color
}

fn hit_sphere(center: Vector3<f32>, radius: f32, ray: &Ray) -> f32 {
    // logic entiryle streamlined to give pixel colors of normales
    let oc: Vector3<f32> = center - ray.get_origin();
    let a = ray.get_direction().dot(&ray.get_direction());
    let b = -2.0 * ray.get_direction().dot(&oc);
    let c = oc.dot(&oc) - radius * radius;
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return -1.0;
    } else {
        let answer = (-b - disc.sqrt()) / (2.0 * a);
        answer
    }
}
