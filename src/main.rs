mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod types;

use rand::Rng;
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
    let material_ground = Arc::new(material::Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(sphere::Sphere::new(
        Point3::new(0.0, -1000.0, -1.0),
        1000.0,
        material_ground,
    )));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0..1.0),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let sphere_material: Arc<dyn material::Material + Send + Sync> = if choose_mat < 0.8
                {
                    // diffuse
                    let albedo = Color::new(
                        rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0),
                    );
                    Arc::new(material::Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::new(
                        rng.gen_range(0.0..0.5),
                        rng.gen_range(0.0..0.5),
                        rng.gen_range(0.0..0.5),
                    );
                    let fuzz = rng.gen_range(0.0..0.5);
                    Arc::new(material::Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Arc::new(material::Dielectric::new(1.5))
                };
                world.add(Arc::new(sphere::Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material_1 = Arc::new(material::Dielectric::new(1.5));
    world.add(Arc::new(sphere::Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 = Arc::new(material::Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(sphere::Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = Arc::new(material::Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(sphere::Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    // Render
    camera.render("target/image.ppm", &world);
}
