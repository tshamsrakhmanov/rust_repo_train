use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // VAR - raw declaration
    let color_type = String::from("P3");
    let x_dim = 500;
    let y_dim = 255;
    let coloring_dim = 255;

    // file connect
    let mut file = File::create("pic.ppm")?;

    // write boilerplate of file type e.t.c...
    write!(file, "{}\n", color_type)?;
    write!(file, "{} {}\n", x_dim, y_dim)?;
    write!(file, "{}\n", coloring_dim)?;

    for y_pos in 0..y_dim {
        let a = y_dim - y_pos;
        println!("Scan lines remaining: {}", a);
        for x_pos in 0..x_dim {
            let r_float: f32 = (x_pos as f32) / ((x_dim as f32) - 1.0);
            let g_float: f32 = (y_pos as f32) / ((y_dim as f32) - 1.0);
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

fn write_pixel_to_file(file: &mut File, r: f32, g: f32, b: f32) {
    let _ = write!(file, "{} {} {}\n", r, g, b);
}
