use crate::aux_fn::{is_face_normal, is_point_on_line};
use nalgebra::Vector3;
use std::fmt;

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
/// Object
/// ********************************************
/// Description of a genereic Object which is consist only of one point in a space.
pub struct Object {
    position: Vector3<f32>,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.logger(f)
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.logger(f)
    }
}

impl Object {
    /// Creates an object via given point
    pub fn new(position: Vector3<f32>) -> Object {
        Object { position: position }
    }
    pub fn logger(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p1 = self.position.x;
        let p2 = self.position.y;
        let p3 = self.position.z;
        let s1 = format!("(x:{}, y:{}, z:{})", p1, p2, p3);
        write!(f, "Object [point: {}]", s1)
    }
}

impl Hittable for Object {
    fn hit_test(&self, ray: &Ray, start: f32, finish: f32) -> HitResultTuple {
        let mut temp_hitrecord = HitRecord::new_default();

        let result_of_check = is_point_on_line(self.position, ray.origin, ray.direction, 0.001);

        if result_of_check.0 {
            temp_hitrecord.distance = result_of_check.1;
            temp_hitrecord.point_of_hit = self.position;
            let a = HitResultTuple::new(true, temp_hitrecord);
            return a;
        }

        let b = HitResultTuple::new(false, temp_hitrecord);
        return b;
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
    fn hit_test(&self, ray: &Ray, start: f32, finish: f32) -> HitResultTuple {
        let a1 = HitRecord::new_default();
        let mut result = HitResultTuple::new(false, a1);
        let mut temp_dist = finish;

        for obj in &self.list_of_objects {
            // println!("Perform test:");
            // println!("{:?}", &ray);
            // println!("{:?}", &obj);
            let assert_object = obj.hit_test(&ray, start, temp_dist);
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
    fn hit_test(&self, ray: &Ray, start: f32, finish: f32) -> HitResultTuple {
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
        // temp_res
        let sqrtd = disc.sqrt();

        let mut root = (h - sqrtd) / a;
        if root <= start || finish <= root {
            root = (h + sqrtd) / a;
            if root <= start || finish <= root {
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
/// TRAITS
/// ********************************************

pub trait Hittable {
    fn hit_test(&self, ray: &Ray, start: f32, finish: f32) -> HitResultTuple;
}
