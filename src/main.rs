mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod types;

use std::ops::Range;
use std::rc::Rc;
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
    let image_width = 1600;

    // Camera
    let camera = camera::Camera::new(aspect_ratio, image_width);

    // World
    let mut world = Box::new(HittableList::new());
    // Materials
    let material_center = Rc::new(material::Dielectric::new(1.5));
    let material_left = Rc::new(material::Dielectric::new(1.5));
    let material_right = Rc::new(material::Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));
    let material_ground = Rc::new(material::Lambertian::new(Color::new(0.8, 0.8, 0.0)));

    world.add(Box::new(sphere::Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(sphere::Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(sphere::Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));
    world.add(Box::new(sphere::Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));

    // Render
    camera.render("target/image.ppm", &world);
}
