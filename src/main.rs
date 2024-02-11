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
    fn at(&self) -> Vector3 {
        self.dir
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

fn main() {
    // Image params
    let image_width = 256;
    let image_height = 256;

    // Image
    let mut image_file = File::create("target/image.ppm").unwrap();

    // Render
    writeln!(image_file, "P3\n{} {}\n255", image_width, image_height).unwrap();

    for j in 0..image_height {
        println!("\rScanlines remaining: {}", (image_height - j));
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0 as f64;

            let pixel = Color::new(r, g, b);
            write_color(&mut image_file, pixel);
        }
    }
    println!("Done");
}
