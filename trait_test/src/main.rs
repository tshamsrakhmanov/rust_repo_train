use nalgebra::Vector3;
use std::f32::{INFINITY, NEG_INFINITY};

use crate::{
    aux_fn::is_point_on_line,
    structures::{Hittable, Object, Ray, World},
};
mod aux_fn;
mod structures;

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
    wrld1.add_object(obj3);
    wrld1.add_object(obj2);
    wrld1.add_object(obj1);

    println!("---------------");
    println!("World in test:");
    println!("{:?}", wrld1);
    println!("---------------");

    // scan for hits:

    let world_by_ray = wrld1.hit_test(&r1, 0.0, INFINITY);

    if world_by_ray.is_hit {
        println!("{:?}", world_by_ray.hit_record);
    } else {
        println!("No hit recored in given world");
    }
}
