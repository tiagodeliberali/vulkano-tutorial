mod clear_image;
mod data_between_buffers;
mod glsl_to_proceess_data;
mod instance_builder;
mod mandelbrot_example;

use clear_image::create_clear_image;
use data_between_buffers::copy_data_example;
use glsl_to_proceess_data::use_glsl_shader;
use mandelbrot_example::create_mandelbrot;

fn main() {
    println!("Copy data between buffers");
    println!("-------------------------");
    copy_data_example();

    println!("Use GLSL shader");
    println!("---------------");
    use_glsl_shader();

    println!("Clear an image and save to disk as clear_image.png");
    println!("--------------------------------------------------");
    create_clear_image();

    println!("Create mandelbrot and save to disk as mandelbrot.png");
    println!("----------------------------------------------------");
    create_mandelbrot();
}
