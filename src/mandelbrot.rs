use image::{ImageBuffer, Rgba, Frame};
use num::Complex;
use rayon::prelude::*;


pub struct ImageOptions {
    pub width: u32,
    pub height: u32,
    pub iterations: u8,
    pub zoom: u32,
    pub x_offset: f64,
    pub y_offset: f64,
}

pub struct AnimationOptions {
    pub zoom_start: u32,
    pub zoom_end: u32,
    pub zoom_step: usize,
}

enum Boundedness {
    Unbounded(u8),
    Bounded,
}

fn mandelbrot_boundedness(c: Complex<f64>, iterations: u8) -> Boundedness {
    let mut z = Complex::<f64> { re: 0.0, im: 0.0 };
    for i in 0..iterations {
        z = z * z + c;
        if z.norm() >= 2.0 {
            return Boundedness::Unbounded(i);
        }
    }
    Boundedness::Bounded
}

pub fn generate_gif(
    image_options: &ImageOptions,
    animation_options: &AnimationOptions
) -> Vec<image::Frame> {
    let ImageOptions {
        width,
        height,
        iterations,
        x_offset,
        y_offset,
        ..
    } = *image_options;

    let AnimationOptions {
        zoom_start,
        zoom_end,
        zoom_step,
    } = *animation_options;

    (zoom_start..zoom_end)
        .step_by(zoom_step)
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|zoom| {
            Frame::new(
                generate_image(
                    &ImageOptions {
                        width,
                        height,
                        iterations,
                        zoom,
                        x_offset,
                        y_offset,
                    },
                    10,
                )
            )
        })
        .collect::<Vec<_>>()
}

pub fn generate_image(
    options: &ImageOptions,
    alpha: u8
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let ImageOptions {
        width,
        height,
        iterations,
        zoom,
        x_offset,
        y_offset,
    } = *options;

    let to_imaginary_domain = |x: u32, y: u32| -> Complex<f64> {
        let re: f64 = f64::from(x) - f64::from(width) / 2.0;
        let im: f64 = f64::from(y) - f64::from(height) / 2.0;
        Complex::<f64> {
            re: re / f64::from(zoom) / f64::from(zoom) + x_offset,
            im: im / f64::from(zoom) / f64::from(zoom) + y_offset
        }
    };

    ImageBuffer::from_fn(width, height, |px, py| {
        match mandelbrot_boundedness(to_imaginary_domain(px, py), iterations) {
            Boundedness::Unbounded(_i) => {
                Rgba::<u8>([0x00, 0x00, 0x00, alpha])
            },
            Boundedness::Bounded => {
                Rgba::<u8>([0xFF, 0xFF, 0xFF, alpha])
            }
        }
    })
}
