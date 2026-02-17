use crate::aux_fn::{
    all_one_vec3, all_zero_vec3, degrees_to_radians, is_face_normal, near_zero,
    random_in_unit_disk, random_on_hemisphere, random_unit_vector, reflect, reflectance, refract,
    write_pixel,
};
use chrono::Local;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use nalgebra::Vector3;
use rand::Rng;
use rayon::prelude::*;
use std::any::Any;
use std::fs::File;
use std::io::prelude::*;
use std::{f32::INFINITY, fmt};

/// ********************************************
/// HitRecord
/// ********************************************
/// Describe finitive fact of hit on an object
///   with need details (where, on which distance, what face)
pub struct HitRecord {
    distance: f32,
    point_of_hit: Vector3<f32>,
    normale: Vector3<f32>,
    is_outside: bool,
    material: Box<dyn Material>,
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
            point_of_hit: all_zero_vec3(),
            normale: all_zero_vec3(),
            is_outside: false,
            material: Box::new(Metal::new(all_zero_vec3(), 1.0)),
        }
    }
    // pub fn get_normale(&self) -> Vector3<f32> {
    //     self.normale
    // }
    // pub fn get_distance(&self) -> f32 {
    //     self.distance
    // }
    // pub fn get_point_of_hit(&self) -> Vector3<f32> {
    //     self.point_of_hit
    // }
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
/// Represent ray in model
/// origin...
/// direciton...
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
    list_of_objects: Vec<Box<dyn Hittable>>,
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

    pub fn add_object(&mut self, object: Box<dyn Hittable>) {
        self.list_of_objects.push(object.into());
    }
    pub fn logger(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut temp_str = String::new();
        for pos in &self.list_of_objects {
            if let Some(a) = pos.as_any().downcast_ref::<Sphere>() {
                let t = format!("Sphere at: {}", a.origin);
                temp_str.push_str(&t);
            } else {
                println!("Unknown object")
            }
            temp_str.push_str(",");
        }
        write!(f, "World [{}]\n", temp_str)
    }
}

impl Hittable for World {
    fn hit_test(&self, ray: &Ray, int: &Interval) -> HitResultTuple {
        let mut result = HitResultTuple::new(false, HitRecord::new_default());
        let mut temp_dist = int.max;

        for obj in &self.list_of_objects {
            let temp_int = Interval::new_by_value(int.min, temp_dist);
            let assert_object = obj.hit_test(&ray, &temp_int);
            if assert_object.is_hit {
                if assert_object.hit_record.distance < temp_dist {
                    temp_dist = assert_object.hit_record.distance;
                    result = assert_object;
                }
            }
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
    material: Box<dyn Material>,
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
    pub fn new(origin: Vector3<f32>, radius: f32, material: Box<dyn Material>) -> Sphere {
        Sphere {
            origin: origin,
            radius: radius,
            material: material,
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

        if let Some(mat) = self.material.as_any().downcast_ref::<Metal>() {
            let albedo_of_material = mat.get_albedo();
            temp_res.hit_record.material = Box::new(Metal::new(
                Vector3::new(
                    albedo_of_material.x,
                    albedo_of_material.y,
                    albedo_of_material.z,
                ),
                mat.fuzz,
            ));
        } else if let Some(mat) = self.material.as_any().downcast_ref::<Lambretian>() {
            let albedo_of_material = mat.get_albedo();
            temp_res.hit_record.material = Box::new(Lambretian::new(Vector3::new(
                albedo_of_material.x,
                albedo_of_material.y,
                albedo_of_material.z,
            )));
        } else if let Some(mat) = self.material.as_any().downcast_ref::<Dielectric>() {
            let refraction_of_material = mat.refraction_index;
            temp_res.hit_record.material = Box::new(Dielectric::new(refraction_of_material));
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
    pub fn _size(&self) -> f32 {
        self.max - self.min
    }
    pub fn _is_contains(&self, x: f32) -> bool {
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
    pub image_width: i32,
    image_height: i32,
    center: Vector3<f32>,
    pixel00loc: Vector3<f32>,
    pixel_delta_u: Vector3<f32>,
    pixel_delta_v: Vector3<f32>,
    samples_per_pixel: i32,
    pixel_sample_scale: f32,
    max_depth: u8,
    defocus_disk_u: Vector3<f32>,
    defocus_disk_v: Vector3<f32>,
    defocus_angle: f32,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: u8,
        lookfrom: Vector3<f32>,
        lookat: Vector3<f32>,
        vfov: f32,
        vup: Vector3<f32>,
        defocus_angle: f32,
        focus_dist: f32,
    ) -> Camera {
        // general constants
        let image_height = ((image_width as f32) / aspect_ratio) as i32;

        let pixel_sample_scale = 1.0 / samples_per_pixel as f32;
        let center = lookfrom;
        // let focal_length: f32 = (lookfrom - lookat).norm();
        let theta = degrees_to_radians(vfov);
        let h = f32::tan(theta / 2.0);
        let viewport_height: f32 = 2.0 * h * focus_dist;
        let viewport_width: f32 = viewport_height * (image_width as f32 / image_height as f32);

        let w = (lookfrom - lookat).normalize();
        let u = (vup.cross(&w)).normalize();
        let v = w.cross(&u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / (image_width as f32);
        let pixel_delta_v = viewport_v / (image_height as f32);

        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * f32::tan(degrees_to_radians(defocus_angle / 2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_width: image_width,
            image_height: image_height,
            center: center,
            pixel00loc: pixel00loc,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v,
            samples_per_pixel: samples_per_pixel,
            pixel_sample_scale: pixel_sample_scale,
            max_depth: max_depth,
            defocus_disk_u: defocus_disk_u,
            defocus_disk_v: defocus_disk_v,
            defocus_angle: defocus_angle,
        }
    }

    /// Main function to call for a draw
    pub fn render(&self, world: &World) -> std::io::Result<()> {
        // start the clock
        let start_time = std::time::Instant::now();

        let buffer_line = (0..self.image_height)
            .cartesian_product(0..self.image_width)
            .collect::<Vec<(i32, i32)>>()
            .into_par_iter()
            .progress_count(self.image_width as u64 * self.image_height as u64)
            .map(|(y, x)| {
                let tmp_color = (0..self.samples_per_pixel)
                    .into_par_iter()
                    .map(|_| {
                        let temp_ray = self.get_ray(x, y as i32);
                        Camera::ray_color(&temp_ray, self.max_depth, world)
                    })
                    .sum::<Vector3<f32>>();

                let temp_pix = write_pixel(tmp_color * self.pixel_sample_scale);

                format!("{}", temp_pix)
            })
            .collect::<Vec<String>>()
            .join("\n");

        // stop the clock
        let end_time = std::time::Instant::now();
        let diff_time = (end_time - start_time).as_secs();

        // prep of file_name
        let formatted_time = format!("{}", Local::now().format("%d_%m_%Y_%H:%M:%S"));
        let file_name = format!(
            "{}@{}x{}@{}rays@{}sec.ppm",
            formatted_time, self.image_width, self.image_height, self.samples_per_pixel, diff_time
        );

        // create file
        let mut file = File::create(&file_name)?;

        // write boilerplate of PPM standard
        write!(file, "{}\n", "P3")?;
        write!(file, "{} {}\n", self.image_width, self.image_height)?;
        write!(file, "{}\n", 255)?;

        // write reslut of the render
        let bytes = buffer_line.as_bytes();
        file.write(bytes)?;

        // debug info at the end to terminal
        println!("--------------");
        println!("Total render time: {:?} sec", diff_time);
        println!(
            "X:{:?}, Y:{:?}, RAYS:{:?}",
            self.get_image_width(),
            self.get_image_heigth(),
            self.get_samples_per_pixel()
        );
        println!("--------------");
        println!("Filename:");
        println!("{}", &file_name);
        println!("--------------");

        Ok(())
    }

    /// Sends given Ray into the world and gives back a color to draw
    ///
    /// Uses fallback (backgroung) and main (some object) colors
    fn ray_color(ray: &Ray, depth: u8, world: &World) -> Vector3<f32> {
        if depth <= 0 {
            return all_zero_vec3();
        }
        // generate test of ray test in world
        let temp_int = Interval::new_by_value(0.001, INFINITY);
        let result = world.hit_test(ray, &temp_int);

        // if hit detected - color the ray in approptirate colors
        if result.is_hit {
            // 1st basic implementation to draw spheres as normales map
            // let a = 0.5 * (result.hit_record.normale + Vector3::new(1.0, 1.0, 1.0));
            // return a;

            // 2nd implementation - just gray scale world based on depth
            // let dir = random_on_hemisphere(result.hit_record.normale + random_unit_vector());
            // let temp_ray = Ray::new(result.hit_record.point_of_hit, dir);
            // return 0.5 * Camera::ray_color(&temp_ray, depth - 1, world);

            // 3rd implementation - give back color based on material paramaters
            //
            let scatter_result = result.hit_record.material.scatter(ray, &result.hit_record);

            if scatter_result.is_scatter {
                let t1: Vector3<f32> =
                    Camera::ray_color(&scatter_result.ray_scattered, depth - 1, world);
                let t2: Vector3<f32> = scatter_result.attenuation;
                let t4 = Vector3::new(t1.x * t2.x, t1.y * t2.y, t1.z * t2.z);
                return t4;
            }

            return all_zero_vec3();
        }

        // if not hit - just draw background
        let unit_direction = ray.get_direction().normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        let bg_color = (1.0 - a) * Vector3::new(0.0, 0.0, 0.0) + a * Vector3::new(0.2, 0.4, 0.7);
        return bg_color;
    }

    /// Gives randomized vector3 in -0.5...+0.5 in XY plane
    fn random_vec_on_xy_plane_in_05_radius() -> Vector3<f32> {
        Vector3::new(
            rand::rng().random_range(0.0..1.0) - 0.5,
            rand::rng().random_range(0.0..1.0) - 0.5,
            0.0,
        )
    }

    pub fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Camera::random_vec_on_xy_plane_in_05_radius();
        let pixel_sample = self.pixel00loc
            + ((i as f32 + offset.x) * self.pixel_delta_u)
            + ((j as f32 + offset.y) * self.pixel_delta_v);

        let ray_origin: Vector3<f32>;

        if self.defocus_angle <= 0.0 {
            ray_origin = self.center;
        } else {
            ray_origin = self.defocus_disk_sample();
        }

        let ray_direction = pixel_sample - ray_origin;
        return Ray::new(ray_origin, ray_direction);
    }

    pub fn defocus_disk_sample(&self) -> Vector3<f32> {
        let p = random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
    fn get_image_width(&self) -> i32 {
        self.image_width
    }
    fn get_image_heigth(&self) -> i32 {
        self.image_height
    }
    fn get_samples_per_pixel(&self) -> i32 {
        // 3rd implementation - give back color based on material paraaaaers
        //
        self.samples_per_pixel
    }
}
/// ********************************************
/// LAMBERTIAN
/// ********************************************
// representation of material type - Lambretian
pub struct Lambretian {
    albedo: Vector3<f32>,
}

impl Lambretian {
    pub fn new(albedo: Vector3<f32>) -> Lambretian {
        Lambretian { albedo }
    }
    pub fn get_albedo(&self) -> Vector3<f32> {
        self.albedo
    }
}

impl Material for Lambretian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> ScatterResult {
        let mut scatter_direction = rec.normale + random_unit_vector();
        if near_zero(&scatter_direction) {
            scatter_direction = rec.normale;
        }
        let scattered = Ray::new(rec.point_of_hit, scatter_direction);
        let attenuation = self.albedo;
        let scatter_result = ScatterResult::new(true, attenuation, scattered);
        scatter_result
    }
}

/// ********************************************
/// SCATTERING RESULT
/// ********************************************
pub struct ScatterResult {
    is_scatter: bool,
    attenuation: Vector3<f32>,
    ray_scattered: Ray,
}

impl ScatterResult {
    pub fn new(is_scatter: bool, attenuation: Vector3<f32>, ray_scattered: Ray) -> ScatterResult {
        ScatterResult {
            is_scatter: is_scatter,
            attenuation: attenuation,
            ray_scattered: ray_scattered,
        }
    }
}

/// ********************************************
/// METAL MATERIAL
/// ********************************************
pub struct Metal {
    albedo: Vector3<f32>,
    fuzz: f32,
}
impl Metal {
    pub fn new(albedo: Vector3<f32>, fuzz: f32) -> Metal {
        Metal { albedo, fuzz }
    }
    pub fn get_albedo(&self) -> Vector3<f32> {
        self.albedo
    }
}
impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> ScatterResult {
        let rnd_vec = random_unit_vector();
        let reflected_vector = reflect(&ray_in.direction, &rec.normale).normalize()
            + (Vector3::new(
                self.fuzz * rnd_vec.x,
                self.fuzz * rnd_vec.y,
                self.fuzz * rnd_vec.z,
            ));
        let scattered_ray = Ray::new(rec.point_of_hit, reflected_vector);
        let attenuation = self.albedo;
        let a2: bool;
        if reflected_vector.dot(&rec.normale) > 0.0 {
            a2 = true;
        } else {
            a2 = false;
        }
        ScatterResult::new(a2, attenuation, scattered_ray)
    }
}

/// ********************************************
/// DIELECTRIC MATERIAL
/// ********************************************
pub struct Dielectric {
    refraction_index: f32,
}
impl Dielectric {
    pub fn new(refcation_index: f32) -> Dielectric {
        Dielectric {
            refraction_index: refcation_index,
        }
    }
}
impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> ScatterResult {
        let attenuation = all_one_vec3();
        let ri: f32;
        if rec.is_outside {
            ri = 1.0 / self.refraction_index
        } else {
            ri = self.refraction_index
        }
        let unit_direction = ray_in.direction.normalize();

        // 1st implementation
        // let refracted = refract(unit_direction, rec.normale, ri);
        // let scattered = Ray::new(rec.point_of_hit, refracted);
        // ScatterResult::new(true, attenuation, scattered)

        // 2nd implementation
        let min_unit_dir = -unit_direction;
        let cos_theta: f32 = (1.0 as f32).min(rec.normale.dot(&min_unit_dir));
        let sin_theta: f32 = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta;
        let refract_codition: bool;
        let reflectance_condition: bool;
        if reflectance(cos_theta, ri) > rand::random_range(0.0..1.0) {
            reflectance_condition = true;
        } else {
            reflectance_condition = false;
        }
        if cannot_refract > 1.0 {
            refract_codition = true;
        } else {
            refract_codition = false;
        }
        let temp_direction: Vector3<f32>;
        if refract_codition || reflectance_condition {
            temp_direction = reflect(&unit_direction, &rec.normale);
        } else {
            temp_direction = refract(unit_direction, rec.normale, ri);
        }

        ScatterResult::new(
            true,
            attenuation,
            Ray::new(rec.point_of_hit, temp_direction),
        )
    }
}

/// ********************************************
/// TRAITS
/// ********************************************

pub trait Hittable: Any {
    fn hit_test(&self, ray: &Ray, int: &Interval) -> HitResultTuple;
}

impl dyn Hittable {
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn _as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait Material: Any {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> ScatterResult;
}

impl dyn Material {
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn _as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

unsafe impl Sync for World {}
