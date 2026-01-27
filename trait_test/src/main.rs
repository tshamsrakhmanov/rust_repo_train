use nalgebra::Vector3;
use std::f32::INFINITY;

fn main() {
    // ray definition
    let ray_origin = Vector3::new(-15.0, 0.0, 0.0);
    let ray_direction = Vector3::new(1000.0, 0.0, 0.0);
    let r1 = Ray::new(ray_origin, ray_direction);

    // objects definition
    let pos1 = Vector3::new(0.0, 0.0, 0.0);
    let pos2 = Vector3::new(10.0, 0.0, 0.0);
    let pos3 = Vector3::new(20.0, 0.0, 0.0);
    let obj1 = Object::new(pos1);
    let obj2 = Object::new(pos2);
    let obj3 = Object::new(pos3);

    // world definition
    let mut wrld1 = World::new();
    wrld1.add_object(obj1);
    wrld1.add_object(obj2);
    wrld1.add_object(obj3);

    println!("---------------");
    println!("World in test:");
    println!("{:?}", wrld1);

    // scan for hits:

    println!("---------------");

    let world_by_ray = wrld1.hit_test(&r1, 0.0, INFINITY);

    if world_by_ray.is_hit {
        println!("{:?}", world_by_ray.hit_record);
    } else {
        println!("No hit recored in give world");
    }
}

#[derive(Debug)]
struct HitRecord {
    distance: f32,
    point_of_hit: Vector3<f32>,
    normale: Vector3<f32>,
    is_outside: bool,
}

impl HitRecord {
    fn new_default() -> HitRecord {
        HitRecord {
            distance: 0.0,
            point_of_hit: Vector3::new(0.0, 0.0, 0.0),
            normale: Vector3::new(0.0, 0.0, 0.0),
            is_outside: false,
        }
    }
    fn new(
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

#[derive(Debug)]
struct Object {
    position: Vector3<f32>,
}

#[derive(Debug)]
struct HitResultTuple {
    is_hit: bool,
    hit_record: HitRecord,
}

impl HitResultTuple {
    fn new(is_hit: bool, hit_record: HitRecord) -> HitResultTuple {
        HitResultTuple {
            is_hit: is_hit,
            hit_record: hit_record,
        }
    }
}

impl Object {
    fn new(position: Vector3<f32>) -> Object {
        Object { position: position }
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

fn is_point_on_line(
    vec_c: Vector3<f32>,
    vec_a: Vector3<f32>,
    vec_b: Vector3<f32>,
    tolerance: f32,
) -> (bool, f32) {
    let vec_d = vec_b - vec_a;
    let vec_e = vec_c - vec_a;
    // 1st check - if point is on the line
    // len of cross prod give a understanding if point is on the line
    let cross_product_length = vec_e.cross(&vec_d).norm();

    if cross_product_length < tolerance {
        // 2nd check
        // dot product give an understanding how much from start point we are with requested point
        let dot_product = vec_d.dot(&vec_e);

        if dot_product > 0.0 {
            let distance_of_hit = vec_e.norm();
            return (true, distance_of_hit);
        } else {
            return (false, -1.0);
        }
    } else {
        return (false, -1.0);
    }
}

#[derive(Debug)]
struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }
}

#[derive(Debug)]
struct World {
    list_of_objects: Vec<Object>,
}

impl World {
    fn new() -> World {
        World {
            list_of_objects: Vec::new(),
        }
    }

    fn add_object(&mut self, object: Object) {
        self.list_of_objects.push(object);
    }
}

impl Hittable for World {
    fn hit_test(&self, ray: &Ray, start: f32, finish: f32) -> HitResultTuple {
        let a1 = HitRecord::new_default();
        let mut a = HitResultTuple::new(false, a1);

        for obj in &self.list_of_objects {
            println!("Perform test:");
            println!("{:?}", &ray);
            println!("{:?}", &obj);
            let assert_object = obj.hit_test(&ray, 0.0, INFINITY);
            if assert_object.is_hit {
                println!("YES hit");
                println!("{:?} record is:", assert_object.hit_record);
                a = assert_object;
            } else {
                println!("NO hit");
            }
            println!("------------");
        }

        a
    }
}

trait Hittable {
    fn hit_test(&self, ray: &Ray, start: f32, finish: f32) -> HitResultTuple;
}
