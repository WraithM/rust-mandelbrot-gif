extern crate num;

use num::Complex;

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let width = lower_right.re - upper_left.re;
    let height = upper_left.im - lower_right.im;

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

pub type Rendering = Vec<Vec<u8>>;

pub fn render(
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Rendering {
    let (width, height) = bounds;
    let mut pixels = vec![vec![0 as u8; width]; height];

    for row in 0..height {
        for col in 0..width {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);

            pixels[row][col] = match escape_time(point, 255) {
                None => 0,
                Some(time) => 255 - time as u8,
            }
        }
    }

    pixels
}

pub fn rendering_to_pixels(rendering: &Rendering) -> Vec<u8> {
    // TODO use .flatten() when it becomes available
    rendering
        .into_iter()
        .flat_map(|v| v.iter().cloned())
        .collect::<Vec<_>>()
}
