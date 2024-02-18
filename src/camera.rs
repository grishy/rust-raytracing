use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
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
        }
    }

    pub fn render(&self, output_path: &str, world: &HittableList) {
        let mut image_file = File::create(output_path).unwrap();
        writeln!(
            image_file,
            "P3\n{} {}\n255",
            self.image_width, self.image_height
        )
        .unwrap();

        let pb = ProgressBar::new(self.image_width as u64 * self.image_height as u64);
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
        );

        for (y, x) in (0..self.image_height).cartesian_product(0..self.image_width) {
            let pixel_center = self.pixel00_loc
                + (x as f64 * self.pixel_delta_u)
                + (y as f64 * self.pixel_delta_v);
            let ray_direction = pixel_center - self.center;

            let r = ray::Ray::new(self.center, ray_direction);

            let pixel_color = ray_color(&r, &world);
            write_color(&mut image_file, pixel_color);
            pb.inc(1);
        }

        pb.finish();
    }
}

fn ray_color(ray: &ray::Ray, world: &HittableList) -> Color {
    let range = Range {
        start: 0.0,
        end: std::f64::MAX,
    };
    match world.hit(ray, range) {
        Some(h) => 0.5 * Color::new(h.normal.x + 1.0, h.normal.y + 1.0, h.normal.z + 1.0),
        None => {
            let unit_direction = na::Unit::new_normalize(ray.dir);
            let a = 0.5 * (unit_direction.y) + 1.0;
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
        }
    }
}

fn write_color(dst: &mut dyn Write, color: Color) {
    // Write the translated [0,255] value of each color component.
    writeln!(
        dst,
        "{} {} {}",
        (255.999 * color[0]) as i32,
        (255.999 * color[1]) as i32,
        (255.999 * color[2]) as i32
    )
    .unwrap();
}
