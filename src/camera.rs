use rand::prelude::*;

use super::ray::Ray;
use super::vec3::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: &Vec3,
        lookat: &Vec3,
        vup: &Vec3,
        vfov_deg: f32,
        aspect: f32,
        aperature: f32,
        focus_dist: f32,
    ) -> Camera {
        let theta = vfov_deg * std::f32::consts::PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);
        Camera {
            lower_left_corner: lookfrom
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2. * half_width * focus_dist * u,
            vertical: 2. * half_height * focus_dist * v,
            origin: *lookfrom,
            u,
            v,
            lens_radius: aperature / 2.,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        }
    }
}

fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(random::<f32>(), random::<f32>(), 0.) - Vec3::new(1., 1., 0.);
        if p.dot(&p) < 1.0 {
            return p;
        }
    }
}
