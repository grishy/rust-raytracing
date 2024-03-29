use std::ops::Range;
use std::sync::Arc;

use crate::material;
use crate::ray;
use crate::types::*;

pub trait Hittable {
    fn hit(&self, ray: &ray::Ray, ray_t: Range<f64>) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vector3,
    pub material: Arc<dyn material::Material + Send + Sync>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    // Called by Sphere::hit
    pub fn set_face_normal(&mut self, ray: &ray::Ray, outward_normal: &Vector3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = ray.dir.dot(&outward_normal) < 0.0;
        // TODO: Avoid clone
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
    }
}
