use nalgebra::Vector3;
use std::fs::File;
use std::io::prelude::*;

use crate::{
    aux_fn::ray_color,
    structures::{Object, Ray, Sphere, World},
};
mod aux_fn;
mod structures;

fn main() -> std::io::Result<()> {
    // BOILER PLATE FROM ORIGINAL PROJECT
    // var declaration
    let image_width = 500;
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_height: i32 = ((image_width as f32) / aspect_ratio) as i32;

    // calculation of view_port and camera
    let focal_length = 1.0;
    let camera_center = Vector3::new(0.0, 0.0, 0.0);
    let viewport_height: f32 = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

    // calculatie viewport vectors
    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    // calculate delta between pixels
    let pixel_delta_u = viewport_u / (image_width as f32);
    let pixel_delta_v = viewport_v / (image_height as f32);

    // calculate location of upper left pixel
    let viewport_upper_left =
        camera_center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // open file
    let mut file = File::create("pic.ppm")?;

    // write boilerplate of file type e.t.c...
    write!(file, "{}\n", "P3")?;
    write!(file, "{} {}\n", image_width, image_height)?;
    write!(file, "{}\n", 255)?;
    // BOILER PLATE FROM ORIGINAL PROJECT

    // ray definition
    // let ray_origin = Vector3::new(-15.0, 0.0, 0.0);
    // let ray_direction = Vector3::new(1000.0, 0.0, 0.0);
    // let r1 = Ray::new(ray_origin, ray_direction);

    // objects definition
    // let pos1 = Vector3::new(0.0, 0.0, -10.0);
    // let pos2 = Vector3::new(0.0, 0.0, -20.0);
    // let pos3 = Vector3::new(0.0, 0.0, -30.0);
    // let obj1 = Object::new(pos1);
    // let obj2 = Object::new(pos2);
    // let obj3 = Object::new(pos3);

    let p1 = Vector3::new(0.0, 0.0, -1.0);
    let r1: f32 = 0.5;
    let s1 = Sphere::new(p1, r1);

    // world definition
    let mut world1 = World::new();
    world1.add_object(s1);
    // world1.add_object(obj3);
    // world1.add_object(obj1);
    // world1.add_object(obj2);

    // println!("---------------");
    // println!("World in test:");
    // println!("{}", world1);
    // println!("---------------");

    // scan for hits:

    // let pixel_color = world1.hit_test(&r1, 0.0, INFINITY);

    // if pixel_color.is_hit {
    //     println!("{}", pixel_color.hit_record);
    // } else {
    //     println!("No hit recored in given world");
    // }

    // fill the space
    for y_pos in 0..image_height {
        let a = image_height - y_pos;
        println!("Scan lines remaining: {}", a);
        for x_pos in 0..image_width {
            let pixel_center =
                pixel00loc + (x_pos as f32 * pixel_delta_u) + (y_pos as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray1 = Ray::new(camera_center, ray_direction);
            let temp_color1 = ray_color(&ray1, &world1);

            // let hit_result = world1.hit_test(&ray1, 0.0, INFINITY);
            // let mut temp_color = Vector3::new(0.0, 0.0, 0.0);
            // if hit_result.is_hit {
            //     temp_color.x = 1.0;
            //     temp_color.y = 1.0;
            //     temp_color.z = 1.0;
            // } else {
            // }
            //
            aux_fn::write_pixel(&mut file, temp_color1);
        }
    }
    Ok(())
}
