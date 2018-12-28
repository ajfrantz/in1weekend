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

fn random_scene() -> HitableList {
    let mut world = HitableList { spheres: Vec::new() };
    // "the ground"
    world.spheres.push(Sphere { center: Vec3::new(0., -1000., 0.), radius: 1000., material: Material::Lambertian { albedo: Vec3::new(0.5, 0.5, 0.5) } });
    // little spheres
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(a as f32 + 0.9 * random::<f32>(), 0.2, b as f32 + 0.9 * random::<f32>());
            if (center - Vec3::new(4., 0.2, 0.)).norm() <= 0.9 { continue; }

            let material = match random::<f32>() {
                x if x < 0.8 => Material::Lambertian { albedo: Vec3::new(random::<f32>()*random::<f32>(), random::<f32>()*random::<f32>(), random::<f32>()*random::<f32>()) },
                x if x < 0.95 => Material::Metal { albedo: Vec3::new(0.5 * (1. + random::<f32>()), 0.5 * (1. + random::<f32>()), 0.5 * (1. + random::<f32>())), fuzz: 0.5 * random::<f32>() },
                _ => Material::Dielectric { refraction_index: 1.5 },
            };
            world.spheres.push(Sphere { center, radius: 0.2, material });
        }
    }
    // the centerpiece spheres
    world.spheres.push(Sphere { center: Vec3::new( 0., 1., 0.), radius: 1., material: Material::Dielectric { refraction_index: 1.5 } });
    world.spheres.push(Sphere { center: Vec3::new(-4., 1., 0.), radius: 1., material: Material::Lambertian { albedo: Vec3::new(0.4, 0.2, 0.1) } });
    world.spheres.push(Sphere { center: Vec3::new( 4., 1., 0.), radius: 1., material: Material::Metal { albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0. } });

    world
}

fn main() {
    let nx = 1920;
    let ny = 1080;
    let ns = 100;

    print!("P3\n{} {}\n255\n", nx, ny);

    let world = random_scene();

    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let up = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.0;
    let aperature = 0.1;
    let camera = Camera::new(&lookfrom, &lookat, &up, 20., nx as f32 / ny as f32, aperature, dist_to_focus);

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
