use nalgebra::Vector3;

fn main() {
    let ray_origin = Vector3::new(-10.0, 0.0, 0.0);
    let ray_direction = Vector3::new(0.0, 0.0, 0.0);
    let r1 = Ray::new(ray_origin, ray_direction);

    let pos1 = Vector3::new(0.0, 0.0, 0.0);
    let pos2 = Vector3::new(10.0, 0.0, 0.0);
    let pos3 = Vector3::new(20.0, 0.0, 0.0);
    let obj1 = Object::new(pos1);
    let obj2 = Object::new(pos2);
    let obj3 = Object::new(pos3);

    let mut wrld1 = World::new();
    wrld1.add_object(obj1);
    wrld1.add_object(obj2);
    wrld1.add_object(obj3);

    println!("{:?}", r1);

    for obj in &wrld1.list_of_objects {
        // println!("{:?}", obj);
        let a = obj.is_hit(&r1);
        if a.is_hit {
            println!("{:?} is hit!", obj);
        } else {
            println!("{:?} NO hit", obj);
        }
    }

    println!("{:?}", wrld1);
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
    fn is_hit(&self, ray: &Ray) -> HitResultTuple {
        let temp_hitrecord = HitRecord::new_default();
        // let mut temp_hit_result_tuple = HitResultTuple::new(false, temp_hitrecord);

        if is_point_on_line(self.position, ray.origin, ray.direction, 0.001) {
            let a = HitResultTuple::new(true, temp_hitrecord);
            return a;
        }

        let b = HitResultTuple::new(false, temp_hitrecord);
        return b;
    }
}

fn is_point_on_line(
    point: Vector3<f32>,
    line_ori: Vector3<f32>,
    line_dir: Vector3<f32>,
    tolerance: f32,
) -> bool {
    // let d = line_dir.normalize();
    // let v = point - line_ori;
    // let distance = v.cross(&d).norm();
    // distance < tolerance
    let direction_vector = line_dir - line_ori;
    // println!("{}", direction_vector);
    let dir_of_point = point - line_ori;
    // println!("{}", dir_of_point);
    let crs_prd = dir_of_point.cross(&direction_vector).norm();
    // println!("{} {} ", crs_prd, tolerance);
    if crs_prd < tolerance {
        return true;
    } else {
        return false;
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

trait Hittable {
    fn is_hit(&self, ray: &Ray) -> HitResultTuple;
}
