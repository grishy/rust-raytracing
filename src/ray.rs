use crate::types::*;

pub struct Ray {
    pub orig: Point3,
    pub dir: Vector3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vector3) -> Ray {
        Ray { orig, dir }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
