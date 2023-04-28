# Bresenham Line Algorithm

This Rust application allows the user to input coordinates for two points on a two-dimensional plane and generates a line between the two points using Bresenham's Line Algorithm. The line is displayed on the terminal screen as a series of 'X' characters.

## Dependencies

This application has the following dependencies:

- `crossterm` crate (version 0.20.0 or later)
- `terminal_size` crate (version 0.1.16 or later)

## Building and Running the Application

To build and run the application, you need to have Rust installed on your system. You can install Rust from the official website at https://www.rust-lang.org/tools/install.

Once Rust is installed, you can build and run the application using the following command in the terminal:

```
cargo run
```

The application will start and wait for input. To input coordinates, enter two points in the format `(x1,y1):(x2,y2)`, where `x1` and `y1` are the coordinates of the first point and `x2` and `y2` are the coordinates of the second point. The application will generate a line between the two points using Bresenham's Line Algorithm and display it on the screen as a series of 'X' characters.

To exit the application, enter 'q' and press Enter.

## License

This application is released under the MIT License. See the LICENSE file for more information.

FYI: I made this for a school project and didnt put too much efffort into it
