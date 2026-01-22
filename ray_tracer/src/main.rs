use std::fs::File;
use std::io::{Bytes, prelude::*};

fn main() -> std::io::Result<()> {
    let mut file = File::create("pic.ppm")?;
    file.write_all(b"P3\n")?;
    file.write_all(b"3 2\n")?;
    file.write_all(b"255\n")?;
    // file.write_all(b"255 0 0 0 255 0 0 0 255\n")?;
    // file.write_all(b"255 255 0 255 255 255 0 0 0\n")?;
    // Ok(())
    //
    let width = 255;
    let height = 255;

    for w_pos in 0..height {
        for h_pos in 0..width {
            let r_float: f32 = (w_pos as f32) / ((width as f32) - 1.0);
            let g_float: f32 = (h_pos as f32) / ((height as f32) - 1.0);
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
