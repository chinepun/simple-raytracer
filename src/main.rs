pub mod vec3;
pub mod color;
pub mod ray;
pub mod sphere;
pub mod hit;


use crate::vec3::*;
use crate::color::*;
use hit::{Hit, World};
use crate::ray::*;
use sphere::Sphere;

fn hit_sphere(center: point3, radius: f32, r: Ray) -> f32
{
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = Vec3::dot(oc, r.direction());
    let c = oc.length_squared() - radius * radius;

    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 { return -1.0; }
    else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: &Ray, world: &World) -> Color
{
    if let Some(rec) = world.hit(r, 0.0, f32::INFINITY)
    {
        0.5 * (rec.normal + Color::new_with_values(1.0, 1.0, 1.0))
    } else {
        let unit_direction = Vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new_with_values(1.0, 1.0, 1.0) + t * Color::new_with_values(0.5, 0.7, 1.0)

    }
    // let center = point3::new_with_values(0.0, 0.0, -1.0);
    // let t = hit_sphere(center, 0.5, r);

    // if t > 0.0
    // {
    //     let N = Vec3::unit_vector(r.at(t) - center);
    //     return 0.5 * Color::new_with_values(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0);
    // }

    // let unit_direction = Vec3::unit_vector(r.direction());
    // let t = 0.5 * (unit_direction.y() + 1.0);

    // (1.0 - t) *
    // Color::new_with_values(1.0, 1.0, 1.0) +
    // t * Color::new_with_values(0.5, 0.7, 1.0)
}

use std::{io::{stdout, Write}, ops::Div};
fn main() 
{
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;

    // World
    let mut world = World::default();
    world.push(Sphere::new(point3::new_with_values(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(point3::new_with_values(0.0, -100.5, -1.0), 100.0));
    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin: point3 = point3::new();
    let horizontal = Vec3::new_with_values(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new_with_values(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2. - vertical / 2. - Vec3::new_with_values(0.0, 0.0, focal_length) ;

    let rows = [0; IMAGE_HEIGHT];
    let columns = [0; IMAGE_WIDTH];

    // Render

    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for (j, _) in rows.iter().enumerate()
    {
        if let Err(_) = stdout().flush() {
            eprintln!("\rScanlines remaining: {}", j)
        };
        for (i, _) in columns.iter().enumerate()
        {
            let u = (i as f32)
                .div((IMAGE_WIDTH - 1) as f32);
            let v = (j as f32)
                .div((IMAGE_HEIGHT - 1) as f32);

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color: Color = ray_color(&r, &world);
            write_color(&pixel_color);
        }
    }
}
