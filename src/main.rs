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

    let length = 12;

    for a in -length..length {
        for b in -length..length {
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
    // Remove and create the render directory
    let output_dir = "render";
    let _ = std::fs::remove_dir_all(output_dir);
    std::fs::create_dir(output_dir).unwrap();

    for step in 0..90 {
        println!("Rendering step {}", step);
        let camera = camera::Camera::new(Point3::new(
            3.0 + (step as f64) / 10.0,
            1.1 + (step as f64 - 2.0).powf(1.3) / 100.0,
            2.0 + (step as f64) / 70.0,
        ));
        let filename = format!("{output_dir}/image_{step}.png");
        camera.render(filename.as_str(), &world);
    }
}
