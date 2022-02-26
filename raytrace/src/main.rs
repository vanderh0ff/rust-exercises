mod vec;
mod ray;
mod hit;
mod sphere;
mod camera;
mod material;

use std::sync::Arc;

use rand::prelude::*;
use rayon::prelude::*;

use vec::{Vec3, Point3, Color};
use ray::Ray;
use hit::{Hit, World};
use sphere::Sphere;
use material::{Lambertian, Metal};
use camera::Camera;

fn random_scene() -> World {
    let mut rng = rand::thread_rng();
    let mut world = World::new();

    let ground_mat = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

    world.push(Box::new(ground_sphere));

    // what is this equals infront of the 11 in the range here
    //  the = makes it inclusive instead of going from 0..10 having 10 entries of 0 - 9 the 0..=10
    //  wold have 11 and include 10
    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new((a as f64) + rng.gen_range(0.0..0.9), 0.2, (b as f64) + rng.gen_range(0.0..0.9));

            if choose_mat < 0.8 {
                let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                let sphere_mat = Arc::new(Lambertian::new(albedo));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else {
                let albedo = Color::random(0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                let sphere = Sphere::new(center, 0.2, sphere_mat);
                
                world.push(Box::new(sphere));

            }
        }
    }

    world
}

fn ray_color (r: &Ray, world: &World, depth: u64) -> Color {
    if depth <= 0 {
        // hit bounce limit no more light collected
        
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() -> () {
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u64 = 1200;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 50;

    let world = random_scene();

    let lookfrom = Point3::new(13.8, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(lookfrom,
                          lookat,
                          vup,
                          20.0,
                          ASPECT_RATIO,
                          aperture,
                          dist_to_focus);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remainint: {}", j + 1);
        // look at this par iter and the map syntax
        let scanline: Vec<Color> = (0..IMAGE_WIDTH).into_par_iter().map(|i| {
            let mut pixel_color = Color::new(0.0,0.0,0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let mut rng = rand::thread_rng();
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_WIDTH - 1) as f64);

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
        pixel_color
        }).collect();
        
        for pixel_color in scanline {
            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }
    eprintln!("Done");

}
