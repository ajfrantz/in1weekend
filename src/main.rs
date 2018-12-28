extern crate rand;

mod camera;
mod hitable;
mod hitable_list;
mod vec3;
mod ray;
mod sphere;

use rand::prelude::*;

use self::camera::Camera;
use self::hitable::Hitable;
use self::hitable_list::HitableList;
use self::vec3::Vec3;
use self::ray::Ray;
use self::sphere::Sphere;

fn color(r: &Ray, world: &HitableList) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.0, std::f32::MAX) {
        0.5 * (hit.normal + 1.)
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
        Sphere { center: Vec3::new(0., 0., -1.), radius: 0.5 },
        Sphere { center: Vec3::new(0., -100.5, -1.), radius: 100. },
    ]};

    let camera = Camera::new();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0., 0., 0.);
            for _s in 0..ns {
                let u = (i as f32 + random::<f32>()) / nx as f32;
                let v = (j as f32 + random::<f32>()) / ny as f32;
                let r = camera.get_ray(u, v);
                col += color(&r, &world);
            }

            let color = col * (255.99 / ns as f32);
            let ir = color.r() as i32;
            let ig = color.g() as i32;
            let ib = color.b() as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
