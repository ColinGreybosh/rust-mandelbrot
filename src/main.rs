mod cli;
mod mandelbrot;

use core::panic;
use mandelbrot::{ImageOptions, generate_image};

fn main() {
    let cli::Args {
        width,
        height,
        x_offset,
        y_offset,
        zoom,
        iterations,
        out_path,
    } = cli::get_args();
    let image_options = &ImageOptions {
        width,
        height,
        iterations,
        zoom,
        x_offset,
        y_offset,
    };

    let image_buffer = generate_image(image_options, 0xFF);
    match image_buffer.save(&out_path) {
        Ok(_) => {
            println!("Successfully saved image to {:#?}.", out_path.as_os_str());
        },
        Err(error) => {
            panic!("Failed to save the image: {:#?}", error);
        }
    };
}
