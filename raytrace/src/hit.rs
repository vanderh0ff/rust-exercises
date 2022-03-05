use std::sync::Arc;

use super::vec::{Point3, Vec3};
use super::ray::Ray;
use super::material::Scatter;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Scatter>,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            (-1.0) * outward_normal
        };
    }
}
// look in to what Vec is
//   a contiguous growable array type with heap allocated contents
// look in to what Box is
//   a box is a pointer type for heap allocation
//   boxes provide ownership and drop ther contents when they go out of scope
//   boxes also ensure you don't over allocate
// look in to what dyn is
//   highlights that calls to methods on the associated trait are dynamically dispatched
//   the complied does not know the concrete type being passed, a dyn trait ref will
//   have two pointers, one to the data and one the the map of method call names to function
//   pointers
//   methods called by dynamic dispatch generally can not be inlined by the compiler
//   dyn traits are more likely to produce smaller code that impl traits
//   as the method wont be duplicated for each concrete type
pub type World = Vec<Box<dyn Hit>>;

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }
        tmp_rec
    }
}

pub trait Hit : Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
