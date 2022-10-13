mod cli;
mod mandelbrot;

fn main() {
    let cli::Args { width, height, iterations, zoom, out } = cli::get_args();
    mandelbrot::generate_image(width, height, iterations, zoom as f32, out);
}
