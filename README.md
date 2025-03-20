# Ray Tracing in Rust

This is a Rust implementation of the ray tracing project described in the book [*Ray Tracing in One Weekend*](https://raytracing.github.io/books/RayTracingInOneWeekend.html). The goal of this project is to port the original C++ code to Rust, leveraging Rust's safety and performance features.

## Overview

The project implements a simple ray tracer that generates a 2D image by simulating the behavior of light rays in a 3D scene. It includes basic features such as:

- Ray-sphere intersection
- Simple camera model
- Background rendering
- Output in PPM image format

The code follows the structure and algorithms outlined in the book, with adaptations to fit Rust's idiomatic style.

## Features

- **Ray Tracing Basics**: Implements rays, spheres, and a simple camera model.
- **Image Rendering**: Outputs a PPM image file to standard output.
- **Rust Idioms**: Uses Rust's type system, error handling, and memory safety features.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) installed on your machine.
- A terminal to run the program.

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/orielsanchez/raytracing.git
   cd raytracing
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

### Running the Program

To render the scene and output the image to a file, run:

```bash
cargo run --release > output.ppm
```

This will generate a PPM image file named `output.ppm`. You can view this file using any PPM-compatible image viewer.

### Example Output

After running the program, you should see an image with a gradient background and a simple sphere rendered in the center.

## Code Structure

The project is organized as follows:

- `src/main.rs`: The entry point of the program. Contains the main rendering loop and camera setup.
- `src/ray.rs`: Implements the `Ray` struct and related functionality.
- `src/vec3.rs`: Defines the `Vec3` struct for 3D vector operations.
- `src/lib.rs`: Handles color calculations and output.

## Contributing

Contributions are welcome! If you'd like to improve the code, fix bugs, or add new features, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [*Ray Tracing in One Weekend*](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by Peter Shirley.
- Thanks to the Rust community for providing excellent tools and resources.
rofessional introduction to your project, making it easy for others to understand, use, and contribute.
