use nalgebra::*;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("pic.ppm")?;
    file.write_all(b"P3\n")?;
    file.write_all(b"255 255\n")?;
    file.write_all(b"255\n")?;

    let x_pos_limit = 255;
    let y_pos_limit = 255;

    for x_pos in 0..x_pos_limit {
        let a = x_pos_limit - x_pos;
        println!("Scan lines remaining: {}", a);
        for y_pos in 0..y_pos_limit {
            let r_float: f32 = (y_pos as f32) / ((x_pos_limit as f32) - 1.0);
            let g_float: f32 = (x_pos as f32) / ((y_pos_limit as f32) - 1.0);
            let b_float: f32 = 0.0;

            let ir = (255.999 * r_float) as u8;
            let ig = (255.999 * g_float) as u8;
            let ib = (255.999 * b_float) as u8;
            let str_to_print = format!("{} {} {}\n", ir, ig, ib);
            let bytes: &[u8] = str_to_print.as_bytes();
            file.write_all(bytes)?;
        }
    }

    Ok(())
}
