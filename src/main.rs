mod cli;
mod mandelbrot;

fn main() {
    let cli::Args { width, height, iterations, zoom, out } = cli::get_args();
    let img = mandelbrot::generate_image(
        width,
        height,
        iterations,
        f64::from(zoom)
    );
    
    match img.save(&out) {
        Ok(_) => {
            println!("Successfully saved image to {:#?}.", out.as_os_str());
        },
        Err(error) => {
            panic!("Failed to save the image: {:#?}", error);
        }
    };
}
