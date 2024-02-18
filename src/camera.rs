use indicatif::ProgressIterator;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use rand::{self, Rng};
use std::fs::File;
use std::ops::Range;

use std::io::prelude::*;

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
    samples_per_pixel: i32, // Count of random samples for each pixel
    max_depth: i32,         // Maximum depth of recursion
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Camera {
        let image_height = (image_width as f64 / aspect_ratio) as i32;

        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        // TODO: Try to use also aspect_ratio here
        // More in '4.2 Sending Rays Into the Scene'
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let camera_center = Point3::new(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = camera_center
            - Vector3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            image_width: image_width,
            image_height: image_height,
            center: camera_center,
            pixel00_loc: pixel00_loc,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v,
            samples_per_pixel: 50,
            max_depth: 10,
        }
    }

    pub fn render(&self, output_path: &str, world: &HittableList) {
        let mut image_file = File::create(output_path).unwrap();

        let pb = ProgressBar::new(self.image_width as u64 * self.image_height as u64);
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
        );

        let pixels: Vec<String> = (0..self.image_height)
            .cartesian_product(0..self.image_width)
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
                pixel_color = Color::new(pixel_color.x.sqrt(), pixel_color.y.sqrt(), pixel_color.z.sqrt());

                pixel_color *= 256.0;
                format!(
                    "{} {} {}",
                    (pixel_color.x.clamp(0., 256.)) as i32,
                    (pixel_color.y.clamp(0., 256.)) as i32,
                    (pixel_color.z.clamp(0., 256.)) as i32
                )
            })
            .progress_with(pb)
            .collect();

        write!(
            image_file,
            "P3
{} {}
{}
{}
",
            self.image_width,
            self.image_height,
            "255", // Max color value
            pixels.join("\n")
        )
        .unwrap();
    }

    // Get a randomly sampled camera ray for the pixel at location i,j.
    fn get_ray(&self, x: i32, y: i32) -> ray::Ray {
        let pixel_center =
            self.pixel00_loc + (x as f64 * self.pixel_delta_u) + (y as f64 * self.pixel_delta_v);
        let pixel_sample = self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = (pixel_center + pixel_sample) - self.center;

        return ray::Ray::new(ray_origin, ray_direction);
    }

    // Returns a random point in the square surrounding a pixel at the origin.
    fn pixel_sample_square(&self) -> Vector3 {
        let mut rng = rand::thread_rng();

        let px = -0.5 + rng.gen_range(0.0..1.0);
        let py = -0.5 + rng.gen_range(0.0..1.0);

        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
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
        Some(h) => {
            match h.material.scatter(ray, &h){
                Some((attenuation, scattered)) => {
                    let color = ray_color(&scattered, depth - 1, world);
                    return Color::new(
                        attenuation.x * color.x,
                        attenuation.y * color.y,
                        attenuation.z * color.z,
                    );
                }
                None => Color::new(0.0, 0.0, 0.0)   
            }

        }
        None => {
            let unit_direction = na::Unit::new_normalize(ray.dir);
            let a = 0.5 * (unit_direction.y) + 1.0;
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
        }
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
