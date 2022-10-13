use image::{ImageBuffer, Rgb};
use num::Complex;

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

pub fn generate_image(
    width: u32,
    height: u32,
    iterations: u8,
    zoom: f64,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let to_imaginary_domain = |x: u32, y: u32| -> Complex<f64> {
        let re: f64 = f64::from(x) - f64::from(width) / 2.0;
        let im: f64 = f64::from(y) - f64::from(height) / 2.0;
        Complex::<f64> {
            re: re / zoom,
            im: im / zoom
        }
    };

    ImageBuffer::from_fn(width, height, |px, py| {
        match mandelbrot_boundedness(to_imaginary_domain(px, py), iterations) {
            Boundedness::Unbounded(_i) => {
                Rgb::<u8>([0x00, 0x00, 0x00])
            },
            Boundedness::Bounded => {
                Rgb::<u8>([0xFF, 0xFF, 0xFF])
            }
        }
    })
}
