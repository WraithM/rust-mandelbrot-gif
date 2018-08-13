extern crate gif;
extern crate mandelbrot;
extern crate num;

use gif::{Encoder, Frame, Repeat, SetParameter};
use mandelbrot::{render, rendering_to_pixels, Rendering};
use num::Complex;
use std::fs::File;

fn write_frame(
    encoder: &mut Encoder<&mut File>,
    width: u16,
    height: u16,
    rendering: Rendering,
) -> std::io::Result<()> {
    let pixels = rendering_to_pixels(&rendering);

    // Convert to grayscale
    let rgb_pixels = pixels
        .into_iter()
        .flat_map(|pixel| vec![pixel, pixel, pixel])
        .collect::<Vec<_>>();
    let frame = Frame::from_rgb(width, height, &rgb_pixels);

    encoder.write_frame(&frame)
}

fn write_gif(filename: &str, renderings: Vec<Rendering>) -> std::io::Result<()> {
    // assuming renderings is non-empty
    let height = renderings[0].len() as u16; // Assuming that all renderings are the same size
    let width = renderings[0][0].len() as u16; // Assuming that all arrays on the inside are the same size

    let mut image = File::create(filename)?;
    let mut encoder = Encoder::new(&mut image, width, height, &[])?;
    encoder.set(Repeat::Infinite)?;

    println!("Created encoder, encoding...");

    for (i, rendering) in renderings.iter().enumerate() {
        println!("Writing frame {}", i);
        write_frame(&mut encoder, width, height, rendering.to_vec())?;
    }

    Ok(())
}

struct MandelbrotBounds {
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
}

struct MandelbrotAnimation {
    width: usize,
    height: usize,
    bounds: Vec<MandelbrotBounds>,
}

fn render_animation(animation: MandelbrotAnimation) -> Vec<Rendering> {
    let bounds = (animation.width, animation.height);

    let mut renderings = Vec::new();

    for bound in animation.bounds {
        renderings.push(render(bounds, bound.upper_left, bound.lower_right));
    }

    renderings
}

fn zoom_bounds(
    width: usize,
    height: usize,
    init_upper_left: Complex<f64>,
    init_lower_right: Complex<f64>,
    scale: f64,
    num_frames: usize,
) -> MandelbrotAnimation {
    let mut upper_left = init_upper_left;
    let mut lower_right = init_lower_right;

    let mut bounds = vec![MandelbrotBounds {
        upper_left,
        lower_right,
    }];

    for _ in 1..num_frames {
        // println!("upper_left: {}", upper_left);
        // println!("lower_right: {}", lower_right);

        let scale_re = num_frames as f64 / scale * (lower_right.re - upper_left.re) / width as f64;
        let scale_im = num_frames as f64 / scale * (upper_left.im - lower_right.im) / height as f64;

        upper_left.re += scale_re;
        upper_left.im -= scale_im;

        lower_right.re -= scale_re;
        lower_right.im += scale_im;

        let bound = MandelbrotBounds {
            upper_left,
            lower_right,
        };

        bounds.push(bound);
    }

    MandelbrotAnimation {
        width,
        height,
        bounds,
    }
}

fn main() {
    let width = 800;
    let height = 600;

    let upper_left = Complex { re: -1.2, im: 0.35 };
    let lower_right = Complex { re: -1.0, im: 0.20 };

    let animation = zoom_bounds(width, height, upper_left, lower_right, 1.0, 20);

    println!("Creating animation...");

    let filename = "target/test.gif";

    write_gif(filename, render_animation(animation))
        .expect(&format!("Failed to write gif: {}!", filename));
}
