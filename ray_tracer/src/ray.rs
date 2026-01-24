pub mod ray_module {
    use nalgebra::{self as na, Vector3};

    pub struct Ray {
        origin: na::Vector3<f32>,
        direction: na::Vector3<f32>,
    }

    impl Ray {
        pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
            Ray { origin, direction }
        }
        pub fn at(&self, t: f32) -> Vector3<f32> {
            let answer = self.origin + t * self.direction;
            answer
        }
        pub fn ray_color(ray: Ray) -> Vector3<f32> {
            let a: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
            a
        }
    }
}
