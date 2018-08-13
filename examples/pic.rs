extern crate image;
extern crate mandelbrot;
extern crate num;

use image::png::PNGEncoder;
use image::ColorType;
use num::Complex;
use std::fs::File;

use mandelbrot::{render, rendering_to_pixels, Rendering};

fn write_image(filename: &str, rendering: &Rendering) -> std::io::Result<()> {
    let height = rendering.len();
    let width = rendering[0].len(); // Assuming that all arrays on the inside are the same size

    let pixels = rendering_to_pixels(rendering);

    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);

    encoder.encode(
        pixels.as_slice(),
        width as u32,
        height as u32,
        ColorType::Gray(8),
    )
}

fn main() {
    let filename = "target/test.png";

    let width = 2000;
    let height = 1000;

    let upper_left = Complex { re: -1.2, im: 0.35 };
    let lower_right = Complex { re: -1.0, im: 0.20 };

    let pixels = render((width, height), upper_left, lower_right);
    write_image(filename, &pixels).expect(&format!("Failed to write image: {}", filename));
}
