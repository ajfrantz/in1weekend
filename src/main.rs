mod vec3;

use self::vec3::Vec3;

fn main() {
    let nx = 200;
    let ny = 100;
    print!("P3\n{} {}\n255\n", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let color = 255.99 * Vec3::new_with_values(i as f32 / nx as f32, j as f32 / ny as f32, 0.2);
            let ir = color.r() as i32;
            let ig = color.g() as i32;
            let ib = color.b() as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
