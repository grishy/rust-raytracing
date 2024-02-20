use image;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use rand::{self, Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::ops::Range;

use crate::hittable::Hittable;
use crate::ray;
use crate::types::*;
use crate::HittableList;

extern crate nalgebra as na;

pub struct Camera {
    image_width: i32,
    image_height: i32,
    center: Point3,         // Camera center
    pixel00_loc: Point3,    // Location of pixel 0, 0
    pixel_delta_u: Vector3, // Offset to pixel to the right
    pixel_delta_v: Vector3, // Offset to pixel below
    defocus_angle: f64,
    defocus_disk_u: Vector3, // Defocus disk horizontal radius
    defocus_disk_v: Vector3, // Defocus disk vertical radius
    samples_per_pixel: i32,  // Count of random samples for each pixel
    max_depth: i32,          // Maximum depth of recursion
}

impl Camera {
    pub fn new(look_from: Point3) -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 600;
        let samples_per_pixel = 200;
        let max_depth = 20;
        let vfov: f64 = 20.0;
        let look_at = Point3::new(0.0, 0.0, 0.0);
        let vup = Vector3::new(0.0, 1.0, 0.0);

        // TODO: Focus does not work
        let defocus_angle = 0.1; // Variation angle of rays through each pixel
        let focus_dist = 10.2; // Distance from camera lookfrom point to plane of perfect focus

        let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();

        let viewport_height = 2. * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let center = look_from;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (look_from - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        // ## Calculate the vectors across the horizontal and down the vertical viewport edges.
        // Vector across viewport horizontal edge
        let viewport_u = viewport_width * u;
        // Vector down viewport vertical edge
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - focus_dist * w - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = (defocus_angle / 2.0 as f64).to_radians().tan() * focus_dist;
        let defocus_disk_u = defocus_radius * u;
        let defocus_disk_v = defocus_radius * v;

        Camera {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn render(&self, output_path: &str, world: &HittableList) {
        let mut imgbuf = image::ImageBuffer::new(self.image_width as u32, self.image_height as u32);

        let pb = ProgressBar::new(self.image_width as u64 * self.image_height as u64);
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
        );

        let pixels = (0..self.image_height)
            .cartesian_product(0..self.image_width)
            .collect::<Vec<(i32, i32)>>()
            .into_par_iter()
            .map(|(y, x)| {
                // Send few rays to the pixel
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(x, y);
                    pixel_color += ray_color(&r, self.max_depth, &world);
                }

                // Divide the color by the number of samples.
                pixel_color /= self.samples_per_pixel as f64;

                // Apply the linear to gamma transform.
                pixel_color = Color::new(
                    pixel_color.x.sqrt(),
                    pixel_color.y.sqrt(),
                    pixel_color.z.sqrt(),
                );

                pixel_color *= 256.0;
                pb.inc(1);
                (y, x, pixel_color)
            })
            .collect::<Vec<(i32, i32, Color)>>();

        for (y,x,pixel) in pixels {
            imgbuf.put_pixel(
                x as u32,
                y as u32,
                image::Rgb([
                    pixel.x as u8,
                    pixel.y as u8,
                    pixel.z as u8,
                ]),
            );
        }

        imgbuf.save(output_path).unwrap();

        pb.finish()
    }

    // Get a randomly-sampled camera ray for the pixel at location i,j, originating from
    // the camera defocus disk.
    fn get_ray(&self, x: i32, y: i32) -> ray::Ray {
        let pixel_center =
            self.pixel00_loc + (x as f64 * self.pixel_delta_u) + (y as f64 * self.pixel_delta_v);
        let pixel_sample = self.pixel_sample_square();

        let ray_origin = if self.defocus_angle < 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = (pixel_center + pixel_sample) - self.center;

        ray::Ray::new(ray_origin, ray_direction)
    }

    // Returns a random point in the camera defocus disk.
    fn defocus_disk_sample(&self) -> Point3 {
        let rd = Camera::random_in_unit_disk();
        self.center + (self.defocus_disk_u * rd.x) + (self.defocus_disk_v * rd.y)
    }

    // Returns a random point in the square surrounding a pixel at the origin.
    fn pixel_sample_square(&self) -> Vector3 {
        let mut rng = rand::thread_rng();

        let px = -0.5 + rng.gen_range(0.0..1.0);
        let py = -0.5 + rng.gen_range(0.0..1.0);

        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn random_in_unit_disk() -> Vector3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.magnitude_squared() < 1.0 {
                return p;
            }
        }
    }
}

fn ray_color(ray: &ray::Ray, depth: i32, world: &HittableList) -> Color {
    let range = Range {
        start: 0.001,
        end: std::f64::MAX,
    };

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    match world.hit(ray, range) {
        Some(h) => match h.material.scatter(ray, &h) {
            Some((attenuation, scattered)) => {
                let color = ray_color(&scattered, depth - 1, world);
                Color::new(
                    attenuation.x * color.x,
                    attenuation.y * color.y,
                    attenuation.z * color.z,
                )
            }
            None => Color::new(0.0, 0.0, 0.0),
        },
        None => {
            let unit_direction = na::Unit::new_normalize(ray.dir);
            let a = 0.8 * (unit_direction.y) + 1.0;
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
        }
    }
}
