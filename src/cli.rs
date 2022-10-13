use clap::Parser;
use std::path::PathBuf;


pub fn get_args() -> Args {
    Args::parse()
}

/// Program to generate an image of the Mandelbrot set.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Width of the output image
    #[arg(
        long,
        default_value_t = 1000,
        value_parser = clap::value_parser!(u32).range(1..),
    )]
    pub width: u32,

    /// Height of the output image
    #[arg(
        long,
        default_value_t = 1000,
        value_parser = clap::value_parser!(u32).range(1..),
    )]
    pub height: u32,

    /// The maximum number of terms calculated in order to determine whether or
    /// not a point is in the Mandelbrot set.
    #[arg(
        short,
        long,
        default_value_t = 100,
        value_parser = clap::value_parser!(u32).range(1..),
    )]
    pub iterations: u32,

    /// Scales the image size
    #[arg(
        short,
        long,
        default_value_t = 200,
        value_parser = clap::value_parser!(u32).range(1..),
    )]
    pub zoom: u32,

    /// Path of the output image file with desired extension.
    #[arg(short, long)]
    pub out: PathBuf,
}
