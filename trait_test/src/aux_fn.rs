use crate::structures::{Interval, Ray};
use nalgebra::Vector3;
use rand::Rng;

pub fn write_pixel(pixel: Vector3<f32>) -> String {
    let int1 = Interval::new_by_value(0.0, 0.999);

    let r = linear_to_gamma(pixel.x);
    let g = linear_to_gamma(pixel.y);
    let b = linear_to_gamma(pixel.z);

    let ir = (256.0 * int1.clamp(r)) as u8;
    let ig = (256.0 * int1.clamp(g)) as u8;
    let ib = (256.0 * int1.clamp(b)) as u8;

    let str1 = format!("{} {} {}\n", ir, ig, ib);
    str1
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
pub fn random_vector_positive() -> Vector3<f32> {
    let a: Vector3<f32> = Vector3::new(
        rand::rng().random_range(0.0..1.0),
        rand::rng().random_range(0.0..1.0),
        rand::rng().random_range(0.0..1.0),
    );
    a
}
pub fn random_vector_by_range(a: f32, b: f32) -> Vector3<f32> {
    let a: Vector3<f32> = Vector3::new(
        rand::rng().random_range(a..b),
        rand::rng().random_range(a..b),
        rand::rng().random_range(a..b),
    );
    a
}
pub fn random_unit_vector() -> Vector3<f32> {
    let ans: Vector3<f32>;
    loop {
        let p = random_vector_by_range(-1.0, 1.0);
        let n = p.magnitude().powi(2);
        if n <= 1.0 {
            ans = p / n.sqrt();
            break;
        }
    }
    ans
}

pub fn random_on_hemisphere(normal: Vector3<f32>) -> Vector3<f32> {
    let on_unit_sphere: Vector3<f32> = random_unit_vector();
    if on_unit_sphere.dot(&normal) > 0.0 {
        return on_unit_sphere;
    } else {
        return -on_unit_sphere;
    }
}
pub fn linear_to_gamma(a: f32) -> f32 {
    if a > 0.0 {
        return a.sqrt();
    }
    return 0.0;
}

/// Check if Vec3 is close to zero
pub fn near_zero(vec: &Vector3<f32>) -> bool {
    let tolerance = 1e-8;
    return vec.x < tolerance && vec.y < tolerance && vec.z < tolerance;
}

pub fn reflect(vec: &Vector3<f32>, normale: &Vector3<f32>) -> Vector3<f32> {
    let temp_answer = vec - 2.0 * vec.dot(&normale) * normale;
    temp_answer
}

pub fn zero_vec3() -> Vector3<f32> {
    Vector3::new(0.0, 0.0, 0.0)
}

pub fn single_vec3() -> Vector3<f32> {
    Vector3::new(1.0, 1.0, 1.0)
}
