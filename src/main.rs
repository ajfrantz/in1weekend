extern crate rand;

mod camera;
mod hitable;
mod hitable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use rand::prelude::*;

use self::camera::Camera;
use self::hitable::Hitable;
use self::hitable_list::HitableList;
use self::material::Material;
use self::ray::Ray;
use self::sphere::Sphere;
use self::vec3::Vec3;

fn color(r: &Ray, world: &HitableList, bounce: i32) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.001, std::f32::MAX) {
        if bounce < 50 {
            if let Some(scatter) = hit.material.scatter(r, &hit.p, &hit.normal) {
                return scatter.attenuation * color(&scatter.ray, world, bounce + 1);
            }
        }
        Vec3::new(0., 0., 0.)
    } else {
        let unit_direction = r.direction.unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.)
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    print!("P3\n{} {}\n255\n", nx, ny);

    let world = HitableList { spheres: vec![
        Sphere { center: Vec3::new(0., 0., -1.), radius: 0.5, material: Material::Lambertian { albedo: Vec3::new(0.1, 0.2, 0.5) } },
        Sphere { center: Vec3::new(0., -100.5, -1.), radius: 100., material: Material::Lambertian { albedo: Vec3::new(0.8, 0.8, 0.0) } },
        Sphere { center: Vec3::new(1., 0., -1.), radius: 0.5, material: Material::Metal { albedo: Vec3::new(0.8, 0.6, 0.2), fuzz: 1.0 } },
        Sphere { center: Vec3::new(-1., 0., -1.), radius: 0.5, material: Material::Dielectric { refraction_index: 1.5 } },
        Sphere { center: Vec3::new(-1., 0., -1.), radius: -0.45, material: Material::Dielectric { refraction_index: 1.5 } },
    ]};

    let camera = Camera::new(&Vec3::new(-2., 2., 1.), &Vec3::new(0., 0., -1.), &Vec3::new(0., 1., 0.), 20., nx as f32 / ny as f32);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0., 0., 0.);
            for _s in 0..ns {
                let u = (i as f32 + random::<f32>()) / nx as f32;
                let v = (j as f32 + random::<f32>()) / ny as f32;
                let r = camera.get_ray(u, v);
                col += color(&r, &world, 0);
            }

            col /= ns as f32;
            let color = 255.99 * Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());
            let ir = color.r() as i32;
            let ig = color.g() as i32;
            let ib = color.b() as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
