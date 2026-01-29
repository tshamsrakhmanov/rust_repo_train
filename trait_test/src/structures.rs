use crate::aux_fn::{is_face_normal, random_on_hemisphere, random_unit_vector, write_pixel};
use nalgebra::Vector3;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::{
    f32::{INFINITY, NEG_INFINITY},
    fmt,
};

/// ********************************************
/// HitRecord
/// ********************************************
/// Used to describe finitive fact of hit on an object
/// Provided with discription: where, how, e.t.c.
pub struct HitRecord {
    distance: f32,
    point_of_hit: Vector3<f32>,
    normale: Vector3<f32>,
    is_outside: bool,
}

impl fmt::Display for HitRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.logger(f)
    }
}

impl fmt::Debug for HitRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.logger(f)
    }
}

impl HitRecord {
    pub fn logger(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p0 = self.point_of_hit.x;
        let p1 = self.point_of_hit.y;
        let p2 = self.point_of_hit.z;
        let p3 = self.normale.x;
        let p4 = self.normale.y;
        let p5 = self.normale.z;
        let s1 = format!("x:{}, y:{}, z:{}", p0, p1, p2);
        let s2 = format!("x:{}, y:{}, z:{}", p3, p4, p5);
        write!(
            f,
            "HitRecord [distance:{}, point_of_hit:({}), normale:({}), is_outside:{}]\n",
            self.distance, s1, s2, self.is_outside
        )
    }
    pub fn new_default() -> HitRecord {
        HitRecord {
            distance: 0.0,
            point_of_hit: Vector3::new(0.0, 0.0, 0.0),
            normale: Vector3::new(0.0, 0.0, 0.0),
            is_outside: false,
        }
    }
    pub fn get_normale(&self) -> Vector3<f32> {
        self.normale
    }
    pub fn get_distance(&self) -> f32 {
        self.distance
    }
    pub fn get_point_of_hit(&self) -> Vector3<f32> {
        self.point_of_hit
    }
    pub fn _new(
        distance: f32,
        point_of_hit: Vector3<f32>,
        normale: Vector3<f32>,
        is_outside: bool,
    ) -> HitRecord {
        HitRecord {
            distance: distance,
            point_of_hit: point_of_hit,
            normale: normale,
            is_outside: is_outside,
        }
    }
}

/// ********************************************
/// HitResultTuple
/// ********************************************
/// When we hit some object and assert a result - this is a tuple which we return back.
/// If object is hit - is_hit: TRUE && hit_record: will be filled by relative data (distance,
/// is_outside e.t.c)
/// If object NOT hit - is_hit: FALSE && hit_record: is return with new_default data (all zeroes)
pub struct HitResultTuple {
    pub is_hit: bool,
    pub hit_record: HitRecord,
}

impl fmt::Display for HitResultTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.logger(f)
    }
}

impl fmt::Debug for HitResultTuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.logger(f)
    }
}

impl HitResultTuple {
    pub fn new(is_hit: bool, hit_record: HitRecord) -> HitResultTuple {
        HitResultTuple {
            is_hit: is_hit,
            hit_record: hit_record,
        }
    }
    pub fn logger(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HitResultTuple[ is_hit: {}, hit_record: {} ]\n",
            self.is_hit, self.hit_record
        )
    }
}

/// ********************************************
/// RAY struct
/// ********************************************

pub struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.logger(f)
    }
}

impl fmt::Debug for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.logger(f)
    }
}
impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }
    pub fn translocate(&self, distance: f32) -> Vector3<f32> {
        let answer = self.origin + distance * self.direction;
        answer
    }
    pub fn get_origin(&self) -> Vector3<f32> {
        self.origin
    }
    pub fn get_direction(&self) -> Vector3<f32> {
        self.direction
    }
    pub fn logger(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p0 = self.origin.x;
        let p1 = self.origin.y;
        let p2 = self.origin.z;
        let p3 = self.direction.x;
        let p4 = self.direction.y;
        let p5 = self.direction.z;
        let s1 = format!("(x:{}, y:{}, z:{})", p0, p1, p2);
        let s2 = format!("(x:{}, y:{}, z:{})", p3, p4, p5);
        write!(f, "Ray [origin: {}, direction: {}]\n", s1, s2)
    }
}

/// ********************************************
/// WORLD struct
/// ********************************************

pub struct World {
    list_of_objects: Vec<Sphere>,
}
impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.logger(f)
    }
}

impl fmt::Debug for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.logger(f)
    }
}
impl World {
    pub fn new() -> World {
        World {
            list_of_objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Sphere) {
        self.list_of_objects.push(object);
    }
    pub fn logger(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s1 = format!("{:?}", self.list_of_objects);
        write!(f, "World [{}]\n", s1)
    }
}

impl Hittable for World {
    fn hit_test(&self, ray: &Ray, int: &Interval) -> HitResultTuple {
        let mut result = HitResultTuple::new(false, HitRecord::new_default());
        let mut temp_dist = int.max;

        for obj in &self.list_of_objects {
            // println!("Perform test:");
            // println!("{:?}", &ray);
            // println!("{:?}", &obj);
            let temp_int = Interval::new_by_value(int.min, temp_dist);
            let assert_object = obj.hit_test(&ray, &temp_int);
            if assert_object.is_hit {
                // println!("YES hit");
                // println!("record is {} ", assert_object.hit_record);
                if assert_object.hit_record.distance < temp_dist {
                    temp_dist = assert_object.hit_record.distance;
                    result = assert_object;
                }
            } else {
                // println!("NO hit");
            }
            // println!(">>> temp_dist:{}", temp_dist);
            // println!("------------");
        }
        result
    }
}

/// ********************************************
/// Sphere
/// ********************************************

pub struct Sphere {
    origin: Vector3<f32>,
    radius: f32,
}
impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.logger(f)
    }
}

impl fmt::Debug for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.logger(f)
    }
}

impl Sphere {
    pub fn new(origin: Vector3<f32>, radius: f32) -> Sphere {
        Sphere {
            origin: origin,
            radius: radius,
        }
    }

    pub fn logger(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p0 = self.origin.x;
        let p1 = self.origin.y;
        let p2 = self.origin.z;
        let s1 = format!("(x:{}, y:{}, z:{})", p0, p1, p2);
        write!(f, "Sphere [origin:{}, radius:{}]\n", s1, self.radius)
    }
}

impl Hittable for Sphere {
    fn hit_test(&self, ray: &Ray, int: &Interval) -> HitResultTuple {
        let mut temp_res = HitResultTuple::new(false, HitRecord::new_default());
        let oc: Vector3<f32> = self.origin - ray.get_origin();
        let a = ray.get_direction().magnitude().powi(2);
        let h = ray.get_direction().dot(&oc);
        let c = oc.magnitude().powi(2) - self.radius * self.radius;

        let disc = h * h - a * c;
        if disc < 0.0 {
            temp_res.is_hit = false;
            return temp_res;
        }

        let sqrtd = disc.sqrt();

        let mut root = (h - sqrtd) / a;

        if !int.is_surround(root) {
            root = (h + sqrtd) / a;
            if !int.is_surround(root) {
                temp_res.is_hit = false;
                return temp_res;
            }
        }

        temp_res.is_hit = true;
        temp_res.hit_record.distance = root;
        temp_res.hit_record.point_of_hit = ray.translocate(root);
        temp_res.hit_record.normale =
            (temp_res.hit_record.point_of_hit - self.origin) / self.radius;
        let n1 = (temp_res.hit_record.point_of_hit - self.origin) / self.radius;
        let check_for_inside_outside = is_face_normal(ray, n1);
        if check_for_inside_outside.0 {
            temp_res.hit_record.normale = n1;
            temp_res.hit_record.is_outside = true;
            // temp_res.hit_record.
        } else {
            temp_res.hit_record.normale = -n1;
            temp_res.hit_record.is_outside = false;
        }

        temp_res
    }
}

/// ********************************************
/// INTERVALS
/// ********************************************
pub struct Interval {
    min: f32,
    max: f32,
}
impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.logger(f)
    }
}

impl Interval {
    pub fn new_by_value(min: f32, max: f32) -> Interval {
        Interval { min: min, max: max }
    }
    pub fn clamp(&self, value: f32) -> f32 {
        if value < self.min {
            return self.min;
        } else if value > self.max {
            return self.max;
        } else {
            return value;
        }
    }

    pub fn logger(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Interval [min:{}, max:{}]\n", self.min, self.max)
    }
    pub fn size(&self) -> f32 {
        self.max - self.min
    }
    pub fn is_contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn is_surround(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }
}

/// ********************************************
/// CAMERA
/// ********************************************
pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: i32,
    image_height: i32,
    center: Vector3<f32>,
    pixel00loc: Vector3<f32>,
    pixel_delta_u: Vector3<f32>,
    pixel_delta_v: Vector3<f32>,
    samples_per_pixel: u8,
    pixel_sample_scale: f32,
    max_depth: u8,
    gamma: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: i32, samples_per_pixel: u8) -> Camera {
        let pixel_sample_scale = 1.0 / samples_per_pixel as f32;
        let image_height = ((image_width as f32) / aspect_ratio) as i32;
        let center = Vector3::new(0.0, 0.0, 0.0);
        let max_depth = 10;
        let gamma = 0.1;

        let focal_length: f32 = 1.0;
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = viewport_height * (image_width as f32 / image_height as f32);
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u / (image_width as f32);
        let pixel_delta_v = viewport_v / (image_height as f32);
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);
        let viewport_upper_left =
            center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            image_height: image_height,
            center: center,
            pixel00loc: pixel00loc,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v,
            samples_per_pixel: samples_per_pixel,
            pixel_sample_scale: pixel_sample_scale,
            max_depth: max_depth,
            gamma: gamma,
        }
    }
    pub fn render(&self, world: &World) -> std::io::Result<()> {
        let mut file = File::create("pic.ppm")?;

        // write boilerplate of file type e.t.c...
        write!(file, "{}\n", "P3")?;
        write!(file, "{} {}\n", self.image_width, self.image_height)?;
        write!(file, "{}\n", 255)?;

        for y_pos in 0..self.image_height {
            let a = self.image_height - y_pos;
            println!("Scan lines remaining: {}", a);
            for x_pos in 0..self.image_width {
                let mut temp_color = Vector3::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let temp_ray = Camera::get_ray(&self, x_pos, y_pos);
                    let new_color = Camera::ray_color(&temp_ray, self.max_depth, world);
                    temp_color = temp_color + new_color;
                }

                write_pixel(&mut file, temp_color * self.pixel_sample_scale);
            }
        }

        Ok(())
    }

    fn ray_color(ray: &Ray, depth: u8, world: &World) -> Vector3<f32> {
        if depth <= 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }
        // generate test of ray test in world
        let temp_int = Interval::new_by_value(0.001, INFINITY);
        let result = world.hit_test(ray, &temp_int);

        // if hit detected - color the ray in approptirate colors
        if result.is_hit && result.hit_record.get_distance() > 0.0 {
            // basic implementation to draw spheres as normales map
            // let a = 0.5 * (result.hit_record.get_normale() + Vector3::new(1.0, 1.0, 1.0));
            // return a;
            let dir = random_on_hemisphere(result.hit_record.get_normale()) + random_unit_vector();
            let temp_ray = Ray::new(result.hit_record.get_point_of_hit(), dir);
            return 0.5 * Camera::ray_color(&temp_ray, depth - 1, world);
        }

        // if not hit - just draw background
        let unit_dicrection = ray.get_direction().normalize();
        let a = 0.5 * (unit_dicrection.y + 1.0);
        let bg_color = (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0);
        return bg_color;
    }
    fn sample_square() -> Vector3<f32> {
        Vector3::new(
            rand::rng().random_range(0.0..1.0) - 0.5,
            rand::rng().random_range(0.0..1.0) - 0.5,
            0.0,
        )
    }
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00loc
            + ((i as f32 + offset.x) * self.pixel_delta_u)
            + ((j as f32 + offset.y) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        return Ray::new(ray_origin, ray_direction);
    }
}

/// ********************************************
/// TRAITS
/// ********************************************

pub trait Hittable {
    fn hit_test(&self, ray: &Ray, int: &Interval) -> HitResultTuple;
}
