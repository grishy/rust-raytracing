use std::ops::Range;

use crate::hittable;
use crate::hittable::length_squared;
use crate::hittable::HitRecord;
use crate::ray;
use crate::types::*;

pub struct Sphere {
    center: Point3,
    radius: f64,
}
impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl hittable::Hittable for Sphere {
    fn hit(&self, ray: &ray::Ray, ray_t: Range<f64>) -> Option<hittable::HitRecord> {
        let oc = ray.orig - self.center;

        let a = length_squared(&ray.dir);
        let half_b = oc.dot(&ray.dir);
        let c = length_squared(&oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.contains(&root) {
                return None;
            }
        }

        let mut hit = HitRecord {
            t: root,
            p: ray.at(root),
            front_face: false,
            normal: Vector3::zeros(),
        };
        let outward_normal = (ray.at(root) - self.center) / self.radius;
        hit.set_face_normal(ray, &outward_normal);

        return Some(hit);
    }
}
