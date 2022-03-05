use std::sync::Arc;
// arc is atomically reference counted
// it provides shared ownership of a type allocated in the heap
// by calling the clone method a new arc instance can be made that 
// points to the same value on the heap as the original arc while
// increasing the reference count. when the last arc pointer to an
// allocation is destroyed then the valued stored in theat allocation
// is also dropped

use super::vec::{Vec3,Point3};
use super::ray::Ray;
use super::hit::{Hit, HitRecord};
use super::material::Scatter;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Scatter> 
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, m: Arc<dyn Scatter>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            mat: m
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length().powi(2);
        let half_b = oc.dot(r.direction());
        let c = oc.length().powi(2) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let root = (-half_b - sqrtd)  / a;
        if root < t_min || t_max < root {
            return None;
        }

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            mat: self.mat.clone(),
            normal: Vec3::new(0.0,0.0,0.0),
            front_face: false
        };

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        // Some indicates that the option is not a None
        // we defined the returntype as an option containing a hit record
        // so we have the None value returned avove and if we do have a value
        // we need to wrap it in the some function so that it is an option type
        Some(rec)
    }
}
