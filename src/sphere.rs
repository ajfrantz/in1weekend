use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(&r.direction);
        let b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant <= 0. {
            return None;
        }

        let discriminant_root = discriminant.sqrt();
        let t = (-b - discriminant_root) / a;
        if (t < t_max) && (t > t_min) {
            let p = r.point_at_parameter(t);
            let normal = (p - self.center) / self.radius;
            return Some(HitRecord {
                t,
                p,
                normal,
                material: self.material,
            });
        }

        let t = (-b + discriminant_root) / a;
        if (t < t_max) && (t > t_min) {
            let p = r.point_at_parameter(t);
            let normal = (p - self.center) / self.radius;
            return Some(HitRecord {
                t,
                p,
                normal,
                material: self.material,
            });
        }

        None
    }
}
