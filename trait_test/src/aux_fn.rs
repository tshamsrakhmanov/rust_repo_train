use crate::structures::Ray;
use nalgebra::Vector3;
use std::fs::File;
use std::io::prelude::*;

// section of useful logic - to check if point in on the line, given by vector
// pub fn is_point_on_line(
//     vec_c: Vector3<f32>,
//     vec_a: Vector3<f32>,
//     vec_b: Vector3<f32>,
//     tolerance: f32,
// ) -> (bool, f32) {
//     // c - testing point
//     // a - start of line
//     // b - end of line
//     let vec_d = vec_b - vec_a;
//     let vec_e = vec_c - vec_a;
//     // 1st check - if point is on the line
//     // len of cross prod give a understanding if point is on the line
//     let cross_product_length = vec_e.cross(&vec_d).norm();
//
//     if cross_product_length < tolerance {
//         // 2nd check
//         // dot product give an understanding how much from start point we are with requested point
//         let dot_product = vec_d.dot(&vec_e);
//
//         if dot_product > 0.0 {
//             let distance_of_hit = vec_e.norm();
//             return (true, distance_of_hit);
//         } else {
//             return (false, -1.0);
//         }
//     } else {
//         return (false, -1.0);
//     }
// }

pub fn write_pixel(file: &mut File, pixel: Vector3<f32>) {
    let ir = (255.999 * pixel.x) as u8;
    let ig = (255.999 * pixel.y) as u8;
    let ib = (255.999 * pixel.z) as u8;

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
