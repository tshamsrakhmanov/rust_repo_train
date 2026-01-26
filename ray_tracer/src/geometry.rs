pub mod geometry {
    use nalgebra::Vector3;

    use crate::Ray;
    use crate::hittable::hittable::{HitRecord, TraitHittableSphere};

    pub struct Sphere {
        center: Vector3<f32>,
        radius: f32,
    }

    impl Sphere {
        pub fn new(center: Vector3<f32>, radius: f32) -> Sphere {
            Sphere {
                center: center,
                radius: radius,
            }
        }
    }

    impl TraitHittableSphere for Sphere {
        fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut HitRecord) -> bool {
            let oc: Vector3<f32> = self.center - ray.get_origin();
            let a = ray.get_direction().magnitude().powi(2);
            let h = ray.get_direction().dot(&oc);
            let c = oc.magnitude().powi(2) - self.radius * self.radius;

            let disc = h * h - a * c;
            if disc < 0.0 {
                return false;
            }

            let sqrtd = disc.sqrt();

            let mut root = (h - sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                root = (h + sqrtd) / a;
                if root <= ray_tmin || ray_tmax <= root {
                    return false;
                }
            }

            rec.set_t(root);
            rec.set_p(ray.at(rec.get_t()));
            rec.set_normal((rec.get_p() - self.center) / self.radius);
            let outward_normal = (rec.get_p() - self.center) / self.radius;
            rec.set_face_normal(ray, outward_normal);

            true
        }
    }
}
