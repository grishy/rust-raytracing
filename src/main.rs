mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod types;

use std::ops::Range;
use std::sync::Arc;
use types::*;

pub struct HittableList {
    objects: Vec<Arc<dyn hittable::Hittable + Send + Sync>>,
}

impl HittableList {
    fn new() -> HittableList {
        HittableList { objects: vec![] }
    }
    fn add(&mut self, object: Arc<dyn hittable::Hittable + Send + Sync>) {
        self.objects.push(object);
    }
}

impl hittable::Hittable for HittableList {
    fn hit(&self, ray: &ray::Ray, ray_t: Range<f64>) -> Option<hittable::HitRecord> {
        let (_closest, hit_record) = self.objects.iter().fold((ray_t.end, None), |acc, item| {
            if let Some(temp_rec) = item.hit(ray, ray_t.start..acc.0) {
                (temp_rec.t, Some(temp_rec))
            } else {
                acc
            }
        });

        hit_record
    }
}

fn main() {
    // Camera
    let camera = camera::Camera::new();

    // World
    let mut world = HittableList::new();

    // Materials
    let material_ground = Arc::new(material::Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(material::Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(material::Dielectric::new(1.5));
    let material_right = Arc::new(material::Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Arc::new(sphere::Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(sphere::Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Arc::new(sphere::Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left.clone(),
    )));
    world.add(Arc::new(sphere::Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Arc::new(sphere::Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Render
    camera.render("target/image.ppm", &world);
}
