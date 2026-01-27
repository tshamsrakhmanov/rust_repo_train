use crate::aux_fn::is_point_on_line;
use nalgebra::Vector3;

#[derive(Debug)]
pub struct HitRecord {
    distance: f32,
    point_of_hit: Vector3<f32>,
    normale: Vector3<f32>,
    is_outside: bool,
}

impl HitRecord {
    pub fn new_default() -> HitRecord {
        HitRecord {
            distance: 0.0,
            point_of_hit: Vector3::new(0.0, 0.0, 0.0),
            normale: Vector3::new(0.0, 0.0, 0.0),
            is_outside: false,
        }
    }
    pub fn new(
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
pub struct Object {
    position: Vector3<f32>,
}

#[derive(Debug)]
pub struct HitResultTuple {
    pub is_hit: bool,
    pub hit_record: HitRecord,
}

impl HitResultTuple {
    pub fn new(is_hit: bool, hit_record: HitRecord) -> HitResultTuple {
        HitResultTuple {
            is_hit: is_hit,
            hit_record: hit_record,
        }
    }
}

impl Object {
    pub fn new(position: Vector3<f32>) -> Object {
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

#[derive(Debug)]
pub struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }
}

#[derive(Debug)]
pub struct World {
    list_of_objects: Vec<Object>,
}

impl World {
    pub fn new() -> World {
        World {
            list_of_objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.list_of_objects.push(object);
    }
}

impl Hittable for World {
    fn hit_test(&self, ray: &Ray, start: f32, finish: f32) -> HitResultTuple {
        let a1 = HitRecord::new_default();
        let mut a = HitResultTuple::new(false, a1);
        let mut temp_dist = finish;

        for obj in &self.list_of_objects {
            println!("Perform test:");
            println!("{:?}", &ray);
            println!("{:?}", &obj);
            let assert_object = obj.hit_test(&ray, start, temp_dist);
            if assert_object.is_hit {
                println!("YES hit");
                println!("record is {:?} ", assert_object.hit_record);
                if assert_object.hit_record.distance < temp_dist {
                    temp_dist = assert_object.hit_record.distance;
                    a = assert_object;
                }
            } else {
                println!("NO hit");
            }
            println!(">>> temp_dist:{}", temp_dist);
            println!("------------");
        }

        a
    }
}

pub trait Hittable {
    fn hit_test(&self, ray: &Ray, start: f32, finish: f32) -> HitResultTuple;
}
