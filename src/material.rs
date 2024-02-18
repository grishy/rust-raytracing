use crate::hittable;
use crate::ray;
use rand::Rng;

use crate::types::*;

pub trait Material {
    fn scatter(
        &self,
        ray_in: &ray::Ray,
        hit_record: &hittable::HitRecord,
    ) -> Option<(Color, ray::Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo: albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &ray::Ray,
        hit_record: &hittable::HitRecord,
    ) -> Option<(Color, ray::Ray)> {
        let mut scatter_direction = hit_record.normal + random_in_unit_sphere();

        // Catch degenerate scatter direction
        // Return true if the vector is close to zero in all dimensions.
        if scatter_direction.x.abs() < f64::EPSILON
            && scatter_direction.y.abs() < f64::EPSILON
            && scatter_direction.z.abs() < f64::EPSILON
        {
            scatter_direction = hit_record.normal;
        }

        let scattered = ray::Ray::new(hit_record.p, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

fn random_in_unit_sphere() -> Vector3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vector3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );
        if p.norm_squared() < 1.0 {
            return p;
        }
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: if fuzz <= 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &ray::Ray,
        hit_record: &hittable::HitRecord,
    ) -> Option<(Color, ray::Ray)> {
        let dir_norm = ray_in.dir.normalize();
        let reflected = dir_norm - 2.0 * dir_norm.dot(&hit_record.normal) * hit_record.normal;
        let scattered = ray::Ray::new(hit_record.p, reflected + self.fuzz * random_in_unit_sphere());
        let attenuation = self.albedo;
        if scattered.dir.dot(&hit_record.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
