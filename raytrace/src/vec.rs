use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Range};
use std::fmt;
use std::fmt::Display;
use rand::prelude::*;

pub struct Vec3 {
    e: [f64; 3]
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2],
        }
    }

    pub fn random(r: Range<f64>) -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3 {
            e: [rng.gen_range(r.clone()),rng.gen_range(r.clone()),rng.gen_range(r.clone())]
        }
    }

    // this is inefficent as it will just keep looping trying to make a new one
    // better logic could be used to go and generate one value then calculate the new
    // valid range for further random values
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::random(-1.0..1.0);
            if v.length < 1.0 {
                return v;
            }
        }
    }

    pub fn  random_in_hemisphere () {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            (-1.0) * in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();

        loop {
            let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0),0.0);
            if p.length < 1.0 {
                return p;
            }
        }
    }

    pub fn x(self) -> f64 {
        self[0]
    }

    pub fn y(self) -> f64 {
        slef[1]
    }

    pub fn z(self) -> f64 {
        slef[2]
    }

    pub fn dot(self, other: Vec3) - f64 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self[1] * other[2] - self[2] * other[1],
                self[2] * other[0] - self[0] * other[2],
                self[0] * other[1] - self[1] * other[0],
            ]
        }
    }
    
    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }

    pub fn refract(self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = ((-1.0) * self).dot(n).min(1.0);
        let r_out_prep = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_prep.lenght().powi(2)).abs().sqrt() * n;
        r_out_prep + r_out_parallel 
    }

    pub fn normailzed(self) -> Vec3 {
        self / self.lenght()
    }

    pub fn near_zero(self) -> bool {
        const EPS: f64 = 1.0e-8;
        self[0].abs < EPS && self[1].abs < EPS && self[2].abs < EPS 
    }

    pub fn format_color(self, samples_per_pixel: u64) -> String {
        let ir = (256.0 * (self[0] / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64;
        let ig = (256.0 * (self[1] / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64;
        let ib = (256.0 * (self[2] / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64;

        format !"{} {} {}", ir, ig, ib)
    }
}
