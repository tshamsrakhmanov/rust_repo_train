pub mod hittable {
    use nalgebra::Vector3;

    use crate::ray::ray_module::Ray;

    pub struct Hittable {
        p: Vector3<f32>,
        normal: Vector3<f32>,
        t: f32,
        front_face: bool,
    }

    impl Hittable {
        pub fn set_p(&mut self, p: Vector3<f32>) {
            self.p = p;
        }
        pub fn set_t(&mut self, t: f32) {
            self.t = t;
        }
        pub fn set_normal(&mut self, normal: Vector3<f32>) {
            self.normal = normal;
        }
        pub fn get_t(&self) -> f32 {
            self.t
        }
        pub fn get_p(&self) -> Vector3<f32> {
            self.p
        }
        pub fn set_face_normal(&mut self, ray: &Ray, outward_normal_unit: Vector3<f32>) {
            let angle = ray.get_direction().dot(&outward_normal_unit);
            if angle < 0.0 {
                self.front_face = true;
            } else {
                self.front_face = false;
            }
            if self.front_face {
                self.normal = outward_normal_unit;
            } else {
                self.normal = -outward_normal_unit;
            }
        }
    }

    pub trait TraitHittable {
        fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut Hittable) -> bool;
    }
}
