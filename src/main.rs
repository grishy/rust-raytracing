use std::fs::File;
use std::io::prelude::*;

extern crate nalgebra as na;
use na::Vector3;

type Color = Vector3<i32>;

fn write_color(dst: &mut dyn Write, color: Color) {
    writeln!(dst, "{} {} {}", color[0], color[1], color[2]).unwrap();
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

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            let pixel = Color::new(ir, ig, ib);
            write_color(&mut image_file, pixel);
        }
    }
    println!("Done");
}
