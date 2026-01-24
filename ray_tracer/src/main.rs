use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // var declaration
    let color_type = String::from("P3");
    let x_dim = 400;
    let y_dim = 200;
    let coloring_dim = 255;

    // file connect
    let mut file = File::create("pic.ppm")?;

    // write boilerplate of file type e.t.c...
    write!(file, "{}\n", color_type)?;
    write!(file, "{} {}\n", x_dim, y_dim)?;
    write!(file, "{}\n", coloring_dim)?;

    // fill the space
    for y_pos in 0..y_dim {
        let a = y_dim - y_pos;
        println!("Scan lines remaining: {}", a);
        for x_pos in 0..x_dim {
            let p = Pixel::new(
                (x_pos as f32) / ((x_dim as f32) - 1.0),
                (y_pos as f32) / ((y_dim as f32) - 1.0),
                0.0,
            );

            write_pixel(&mut file, p);
        }
    }
    println!("Done!");

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
