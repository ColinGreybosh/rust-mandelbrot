# rust-mandelbrot
A CLI for generating images of the Mandelbrot set, written in Rust.

## Installation
* Follow [the steps](https://www.rust-lang.org/tools/install) to install Rust.
* Clone this repository.
* In the repository root, run `cargo run -r -- [OPTIONS] --out <OUT>`.

## CLI
```
Usage: mandelbrot [OPTIONS] --out <OUT>

Options:
      --width <WIDTH>            Width of the output image [default: 1000]
      --height <HEIGHT>          Height of the output image [default: 1000]
  -i, --iterations <ITERATIONS>  The maximum number of terms calculated in order to determine whether or not a point is in the Mandelbrot set [default: 100]
  -z, --zoom <ZOOM>              Scales the image size [default: 200]
  -o, --out <OUT>                Path of the output image file with desired extension
  -h, --help                     Print help information
  -V, --version                  Print version information
```
