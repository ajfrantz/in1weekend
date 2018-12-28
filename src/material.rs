use rand::prelude::*;

use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
}

pub struct Scattering {
    pub ray: Ray,
    pub attenuation: Vec3,
}

impl Material {
    pub fn scatter(
        &self,
        incident_ray: &Ray,
        position: &Vec3,
        normal: &Vec3,
    ) -> Option<Scattering> {
        match self {
            Material::Lambertian { albedo } => {
                let target = position + normal + random_in_unit_sphere();
                let scattered = Ray {
                    origin: *position,
                    direction: target - position,
                };
                Some(Scattering {
                    ray: scattered,
                    attenuation: *albedo,
                })
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = reflect(&incident_ray.direction.unit(), normal);
                let scattered = Ray {
                    origin: *position,
                    direction: reflected + *fuzz * random_in_unit_sphere(),
                };
                if scattered.direction.dot(normal) > 0. {
                    Some(Scattering {
                        ray: scattered,
                        attenuation: *albedo,
                    })
                } else {
                    None
                }
            }
        }
    }
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(random::<f32>(), random::<f32>(), random::<f32>())
            - Vec3::new(1., 1., 1.);
        if p.squared_norm() < 1.0 {
            return p;
        }
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return v - 2. * v.dot(n) * n;
}
