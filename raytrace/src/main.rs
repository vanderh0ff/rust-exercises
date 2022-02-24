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
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), ground_mat);

    world.push(Box::new(ground_sphere));

    // what is this equals infront of the 11 in the range here
    //  the = makes it inclusive instead of going from 0..10 having 10 entries of 0 - 9 the 0..=10
    //  wold have 11 and include 10
    for a in -11..=11

}
