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
    ///
    /// Must be a positive integer.
    #[arg(
        long,
        default_value_t = 1000,
        value_parser = clap::value_parser!(u32).range(1..),
    )]
    pub width: u32,

    /// Height of the output image
    ///
    /// Must be a positive integer.
    #[arg(
        long,
        default_value_t = 1000,
        value_parser = clap::value_parser!(u32).range(1..),
    )]
    pub height: u32,

    /// The maximum number of terms calculated in order to determine whether or
    /// not a point is in the Mandelbrot set.
    ///
    /// Must be a positive integer.
    #[arg(
        short,
        long,
        default_value_t = 100,
        value_parser = clap::value_parser!(u8).range(1..),
    )]
    pub iterations: u8,

    /// Scales the image size
    ///
    /// Must be a positive integer.
    #[arg(
        short,
        long,
        default_value_t = 15,
        value_parser = clap::value_parser!(u32).range(1..),
    )]
    pub zoom: u32,

    /// The offset of the image along the X axis
    ///
    /// Must be a floating point number.
    #[arg(
        short,
        long,
        default_value_t = -0.75,
    )]
    pub x_offset: f64,

    /// The offset of the image along the Y axis
    ///
    /// Must be a floating point number.
    #[arg(
        short,
        long,
        default_value_t = 0.1,
    )]
    pub y_offset: f64,

    /// Path of the output image file with desired extension.
    #[arg(short, long)]
    pub out_path: PathBuf,
}
