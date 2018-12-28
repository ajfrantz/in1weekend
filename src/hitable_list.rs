use super::hitable::{HitRecord, Hitable};
use super::ray::Ray;
use super::sphere::Sphere;
use std::vec::Vec;

pub struct HitableList {
    pub spheres: Vec<Sphere>,
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        for sphere in self.spheres.iter() {
            if let Some(hit) = sphere.hit(r, t_min, closest_hit.map_or(t_max, |h| h.t)) {
                closest_hit = Some(hit);
            }
        }
        closest_hit
    }
}
