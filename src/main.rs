use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Range;

extern crate nalgebra as na;

type Point3 = na::Point3<f64>;
type Vector3 = na::Vector3<f64>;
type Color = na::Vector3<f64>;

fn length_squared(v: &Vector3) -> f64 {
    v.x * v.x + v.y * v.y + v.z * v.z
}

struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    image_height: i32,
    center: Point3,         // Camera center
    pixel00_loc: Point3,    // Location of pixel 0, 0
    pixel_delta_u: Vector3, // Offset to pixel to the right
    pixel_delta_v: Vector3, // Offset to pixel below
}

impl Camera {
    fn new(aspect_ratio: f64, image_width: i32) -> Camera {
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
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            image_height: image_height,
            center: camera_center,
            pixel00_loc: pixel00_loc,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v,
        }
    }

    fn render(&self, output_path: &str, world: &hittable_list) {
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

            let r = Ray::new(self.center, ray_direction);

            let pixel_color = ray_color(&r, &world);
            write_color(&mut image_file, pixel_color);
            pb.inc(1);
        }

        pb.finish();
    }
}

struct Ray {
    orig: Point3,
    dir: Vector3,
}

impl Ray {
    fn new(orig: Point3, dir: Vector3) -> Ray {
        Ray {
            orig: orig,
            dir: dir,
        }
    }
    fn origin(&self) -> Point3 {
        self.orig
    }
    fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}

struct hit_record {
    p: Point3,
    normal: Vector3,
    t: f64,
    front_face: bool,
}

impl hit_record {
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = ray.dir.dot(&outward_normal) < 0.0;
        // TODO: Avoid clone
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
    }
}

trait hittable {
    fn hit(&self, ray: &Ray, ray_t: Range<f64>) -> Option<hit_record>;
}

struct hittable_list {
    objects: Vec<Box<dyn hittable>>,
}

impl hittable_list {
    fn new() -> hittable_list {
        hittable_list { objects: vec![] }
    }
    fn add(&mut self, object: Box<dyn hittable>) {
        self.objects.push(object);
    }
}

impl hittable for hittable_list {
    fn hit(&self, ray: &Ray, ray_t: Range<f64>) -> Option<hit_record> {
        self.objects
            .iter()
            .flat_map(|obj| obj.hit(ray, ray_t.clone()))
            .next()
    }
}

struct Sphere {
    center: Point3,
    radius: f64,
}
impl Sphere {
    fn new(center: Point3, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Range<f64>) -> Option<hit_record> {
        let oc = ray.origin() - self.center;

        let a = length_squared(&ray.dir);
        let half_b = oc.dot(&ray.dir);
        let c = length_squared(&oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.contains(&root) {
                return None;
            }
        }

        let mut hit = hit_record {
            t: root,
            p: ray.at(root),
            front_face: false,
            normal: Vector3::zeros(),
        };
        let outward_normal = (ray.at(root) - self.center) / self.radius;
        hit.set_face_normal(ray, &outward_normal);

        return Some(hit);
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

fn ray_color(ray: &Ray, world: &hittable_list) -> Color {
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

fn main() {
    // Params
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Camera
    let camera = Camera::new(aspect_ratio, image_width);

    // World
    let mut world = Box::new(hittable_list::new());
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.1, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Render
    camera.render("target/image.ppm", &world);
}
