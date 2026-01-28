use crate::structures::{Interval, Ray};
use nalgebra::{Vector3, clamp};
use std::fs::File;
use std::io::prelude::*;

pub fn write_pixel(file: &mut File, pixel: Vector3<f32>) {
    let int1 = Interval::new_by_value(0.0, 0.999);

    let ir = (256.0 * int1.clamp(pixel.x)) as u8;
    let ig = (256.0 * int1.clamp(pixel.y)) as u8;
    let ib = (256.0 * int1.clamp(pixel.z)) as u8;

    let str1 = format!("{} {} {}\n", ir, ig, ib);
    let byt1 = str1.as_bytes();
    let _ = file.write(byt1);
}

pub fn is_face_normal(ray: &Ray, outward_normal_unit: Vector3<f32>) -> (bool, Vector3<f32>) {
    let answer_bool;
    let answer_vec;
    let angle = ray.get_direction().dot(&outward_normal_unit);

    if angle < 0.0 {
        answer_bool = true;
    } else {
        answer_bool = false;
    }

    if answer_bool {
        answer_vec = outward_normal_unit;
    } else {
        answer_vec = -outward_normal_unit;
    }

    (answer_bool, answer_vec)
}
