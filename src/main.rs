mod hitable;
mod hitable_list;
mod vec3;
mod ray;
mod sphere;

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
    print!("P3\n{} {}\n255\n", nx, ny);

    let lower_left_corner = Vec3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2., 0.);
    let origin = Vec3::new(0., 0., 0.);

    let world = HitableList { spheres: vec![
        Sphere { center: Vec3::new(0., 0., -1.), radius: 0.5 },
        Sphere { center: Vec3::new(0., -100.5, -1.), radius: 100. },
    ]};

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray { origin, direction: lower_left_corner + u * horizontal + v * vertical };
            let color = 255.99 * color(&r, &world);
            let ir = color.r() as i32;
            let ig = color.g() as i32;
            let ib = color.b() as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
