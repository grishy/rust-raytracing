use std::fs::File;
use std::io::prelude::*;

extern crate nalgebra as na;

type Point3 = na::Point3<f64>;
type Vector3 = na::Vector3<f64>;
type Color = na::Vector3<f64>;

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
    fn at(&self, t: f64) -> Vector3 {
        self.orig.coords + t * self.dir
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

fn ray_color(ray: &Ray) -> Color {
    // Get unit_directrion from ray
    let unit_direction = na::Unit::new_normalize(ray.dir);
    let a = 0.5 * (unit_direction.y) + 1.0;

    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Params
    let aspect_ratio = 16.0 / 9.0;

    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let focal_length = 2.0;
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
    let viewport_upper_left =
        camera_center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    let mut image_file = File::create("target/image.ppm").unwrap();
    writeln!(image_file, "P3\n{} {}\n255", image_width, image_height).unwrap();

    for j in 0..image_height {
        println!("\rScanlines remaining: {}", (image_height - j));
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;

            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r);
            write_color(&mut image_file, pixel_color);
        }
    }
    println!("Done");
}
