use image::{ImageBuffer, Rgb};
use num::Complex;
use std::path::PathBuf;


pub fn generate_image(width: u32, height: u32, iterations: u32, zoom: f32, out: PathBuf) {
    let to_imaginary_domain = |x: u32, y: u32| -> (f32, f32) {
        let re: f32 = x as f32 - width as f32 / 2.0;
        let im: f32 = y as f32 - height as f32 / 2.0;
        (re / zoom, im / zoom)
    };
    
    println!("Generating {} x {} image of the Mandelbrot set...", width, height);
    
    let img = ImageBuffer::from_fn(width, height, |px, py| {
        let (x, y) = to_imaginary_domain(px, py);
        let c = Complex::<f32> { re: x, im: y };
        let mut z = Complex::<f32> { re: 0.0, im: 0.0 };
        for _i in 0..iterations {
            z = z * z + c;
            if z.norm() >= 2.0 {
                let r = (px  / width) as u8;
                let b = (py / height) as u8;
                return Rgb::<u8>([r, 0x00, b]);
            }
        }
        Rgb::<u8>([0xFF, 0xFF, 0xFF])
    });
    
    match img.save(out.clone()) {
        Ok(_) => {
            println!("Successfully saved image to {:#?}.", out.as_os_str());
        },
        Err(error) => {
            panic!("Failed to save the image: {:#?}", error);
        }
    };
}
