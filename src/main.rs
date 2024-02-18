mod camera;
mod hittable;
mod ray;
mod sphere;
mod types;

use std::ops::Range;
use types::*;

pub struct HittableList {
    objects: Vec<Box<dyn hittable::Hittable>>,
}

impl HittableList {
    fn new() -> HittableList {
        HittableList { objects: vec![] }
    }
    fn add(&mut self, object: Box<dyn hittable::Hittable>) {
        self.objects.push(object);
    }
}

impl hittable::Hittable for HittableList {
    fn hit(&self, ray: &ray::Ray, ray_t: Range<f64>) -> Option<hittable::HitRecord> {
        self.objects
            .iter()
            .flat_map(|obj| obj.hit(ray, ray_t.clone()))
            .next()
    }
}

fn main() {
    // Params
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 600;

    // Camera
    let camera = camera::Camera::new(aspect_ratio, image_width);

    // World
    let mut world = Box::new(HittableList::new());
    world.add(Box::new(sphere::Sphere::new(
        Point3::new(0.0, 0.1, -1.0),
        0.5,
    )));
    world.add(Box::new(sphere::Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Render
    camera.render("target/image.ppm", &world);
}
