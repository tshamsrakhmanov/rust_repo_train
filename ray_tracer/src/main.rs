use std::fs::File;
use std::io::prelude::*;

use nalgebra::Vector3;

mod ray;

fn main() -> std::io::Result<()> {
    // var declaration
    let image_width = 500;
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_height: i32 = ((image_width as f32) / aspect_ratio) as i32;

    // calculation of view_port
    let viewport_height: f32 = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

    // file connect
    let mut file = File::create("pic.ppm")?;

    // write boilerplate of file type e.t.c...
    write!(file, "{}\n", "P3")?;
    write!(file, "{} {}\n", image_width, image_height)?;
    write!(file, "{}\n", 255)?;

    // test
    // let p0: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
    // let p1: Vector3<f32> = Vector3::new(1.0, 1.0, 1.0);
    // let r1 = ray::Ray_module::Ray::new(p0, p1);
    // let r2 = r1.at(23.0);

    // fill the space
    for y_pos in 0..image_height {
        let a = image_height - y_pos;
        println!("Scan lines remaining: {}", a);
        for x_pos in 0..image_width {
            let p = Pixel::new(
                (x_pos as f32) / ((image_width as f32) - 1.0),
                (y_pos as f32) / ((image_height as f32) - 1.0),
                0.0,
            );

            write_pixel(&mut file, p);
        }
    }
    println!("Done!");

    println!("{} {} ", viewport_width, viewport_height);

    Ok(())
}

fn write_pixel(file: &mut File, pixel: Pixel) {
    let ir = (255.999 * pixel.r) as u8;
    let ig = (255.999 * pixel.g) as u8;
    let ib = (255.999 * pixel.b) as u8;

    let str1 = format!("{} {} {}\n", ir, ig, ib);
    let byt1 = str1.as_bytes();
    let _ = file.write(byt1);
}

struct Pixel {
    r: f32,
    g: f32,
    b: f32,
}

impl Pixel {
    fn new(r: f32, g: f32, b: f32) -> Pixel {
        Pixel { r: r, g: g, b: b }
    }
}
