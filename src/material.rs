use rand::prelude::*;

use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric { refraction_index: f32 },
}

#[derive(Debug, Copy, Clone)]
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
            Material::Dielectric { refraction_index } => {
                let r_dot_n = incident_ray.direction.dot(normal);
                let (outward_normal, ni_over_nt, cosine) = if r_dot_n > 0. {
                    let cosine = refraction_index * r_dot_n / incident_ray.direction.norm();
                    (-normal, *refraction_index, cosine)
                } else {
                    let cosine = -r_dot_n / incident_ray.direction.norm();
                    (*normal, 1.0 / refraction_index, cosine)
                };

                let reflected_dir = reflect(&incident_ray.direction, normal);
                let attenuation = Vec3::new(1., 1., 1.);
                let reflection = Some(Scattering {
                    ray: Ray {
                        origin: *position,
                        direction: reflected_dir,
                    },
                    attenuation,
                });

                refract(&incident_ray.direction, &outward_normal, ni_over_nt).map_or(
                    reflection,
                    |refracted| {
                        if random::<f32>() < schlick(cosine, *refraction_index) {
                            reflection
                        } else {
                            Some(Scattering {
                                ray: Ray {
                                    origin: *position,
                                    direction: refracted,
                                },
                                attenuation,
                            })
                        }
                    },
                )
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

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.unit();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}
