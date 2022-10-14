# rust-mandelbrot
A CLI for generating images of the Mandelbrot set, written in Rust.

## Example Usage
```
$ cargo run -r -- --out-path output/blah.png --width 2000 --height 2000 --zoom 400
   Compiling mandelbrot v0.1.0 (/Users/colin/mandelbrot)
    Finished release [optimized] target(s) in 0.71s
     Running `target/release/mandelbrot --out-path output/blah.png --width 2000 --height 2000 --zoom 400`
Successfully saved image to "output/blah.png".
```

### Output Image
![test1](https://user-images.githubusercontent.com/18294604/195649245-b0011236-0388-4c8d-b10a-12b01f1efeee.png)

## Installation
* [Install Rust](https://www.rust-lang.org/tools/install).
* Clone this repository.
* In the repository root, run `cargo run -r -- [OPTIONS] --out <OUT>`.

## CLI
```
Usage: mandelbrot [OPTIONS] --out-path <OUT_PATH>

Options:
      --width <WIDTH>            Width of the output image [default: 1000]
      --height <HEIGHT>          Height of the output image [default: 1000]
  -i, --iterations <ITERATIONS>  The maximum number of terms calculated in order to determine whether or not a point is in the Mandelbrot set [default: 100]
  -z, --zoom <ZOOM>              Scales the image size [default: 15]
  -x, --x-offset <X_OFFSET>      The offset of the image along the X axis [default: -0.75]
  -y, --y-offset <Y_OFFSET>      The offset of the image along the Y axis [default: 0.1]
  -o, --out-path <OUT_PATH>      Path of the output image file with desired extension
  -h, --help                     Print help information (use `--help` for more detail)
  -V, --version                  Print version information
```
