use std::cmp::max;

use image::{self, Rgb};
use image::{ImageBuffer, RgbImage};

use imageproc;
use imageproc::drawing;
use imageproc::point::Point;

fn main() {
    let white = Rgb([255, 255, 255]);
    let black = Rgb([0, 0, 0]);

    // The dimension of the output image
    let dims = (3840, 2160);

    // The minimum area of each triangle drawn. If there was no cutoff for drawing
    // triangles, then this would run forever. If the cutoff is too small, the result is
    // not pleasing to the eyes as the individual triangles are too small and it looks
    // like noise
    let min_area = max(dims.0 * dims.1 / 32000, 32);

    draw_sierpinski_triangle(dims, min_area, white, black);
}

/// A simple struct to hold the coordinates of each vertex of a triangle in the image
struct Triangle {
    /// The three points that represent the coordinates of the vertices of the triangle
    points: [Point<i32>; 3],
}

/// A helper function to calculate the area of a triangle based solely on the coordinates
/// of its vertices. This formula comes from taking half the determinant of a matrix set
/// up like:
/// ```
///     1   |  a_x  a_y  1  |
/// A = _ * |  b_x  b_y  1  |
///     2   |  c_x  c_y  1  |
/// ```
///
/// # Arguments
///
/// * `triangle: &Triangle` - A triangle to calculate the area of
fn area(triangle: &Triangle) -> i32 {
    let v = triangle.points;
    let (a, b, c) = (v[0], v[1], v[2]);

    let ar = (a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y)) / 2;

    // You have to take the absolute value of the triangle because handedness comes into
    // play with the determinant but the magnitude always equals the area
    return ar.abs();
}

/// A function to find the midpoint of two points
fn midpoint(point_1: &Point<i32>, point_2: &Point<i32>) -> Point<i32> {
    let mid_x = (point_1.x + point_2.x) / 2;
    let mid_y = (point_1.y + point_2.y) / 2;

    Point { x: mid_x, y: mid_y }
}

/// Returns a tuple of (center_triangle, corner_triangles)
fn subdivide_triangle(triangle: &Triangle) -> (Triangle, Vec<Triangle>) {
    let [a, b, c] = triangle.points;
    let [ab, ac, bc] = [midpoint(&a, &b), midpoint(&a, &c), midpoint(&b, &c)];

    return (
        Triangle {
            points: [ab, ac, bc],
        },
        vec![
            Triangle {
                points: [a, ab, ac],
            },
            Triangle {
                points: [b, ab, bc],
            },
            Triangle {
                points: [c, ac, bc],
            },
        ],
    );
}

/// Given the dimensions of an image to draw on, return the largest equilateral triangle
/// that fits on screen and is centered at the center of the screen
fn max_centered_eq_triangle(dims: (i32, i32)) -> Triangle {
    let (x, y) = (dims.0 as f32, dims.1 as f32);
    let dims_center = (x / 2.0, y / 2.0);

    let sqrt3over2 = f32::sqrt(3.0) / 2.0;

    // Here, I'm trying to find if the height or width is the constraining dimension.
    // If it's the height, work backwards to find a side length,
    // If it's the width, use that as the side length
    let side_length = if x * sqrt3over2 > y {
        y / sqrt3over2
    } else {
        x
    };

    let top_center = Point {
        x: dims_center.0 as i32,
        y: (dims_center.1 - side_length * sqrt3over2 / 2.0) as i32,
    };

    let bottom_left = Point {
        x: (dims_center.0 - side_length / 2.0) as i32,
        y: (dims_center.1 + side_length * sqrt3over2 / 2.0) as i32,
    };

    let bottom_right = Point {
        x: (dims_center.0 + side_length / 2.0) as i32,
        y: (dims_center.1 + side_length * sqrt3over2 / 2.0) as i32,
    };

    Triangle {
        points: [top_center, bottom_left, bottom_right],
    }
}

/// Draw a sierpinski triangle
///
/// # Arguments
///
/// * `dims: (i32, i32)` - The size of the image to draw
///
/// * `min_area: i32` - The minimum area for the triangles when drawing. If this value is
/// too small it produces unwanted artifacts in the image
///
/// * `base_color: Rgb<u8>` - The color of the pixels that are in the sierpinski triangle
///
/// * `empty_color: Rgb<u8>` - The color of pixels not in the sierpinski triangle
fn draw_sierpinski_triangle(
    dims: (i32, i32),
    min_area: i32,
    base_color: Rgb<u8>,
    empty_color: Rgb<u8>,
) {
    let mut triangle_vec: Vec<Triangle> = Vec::new();

    let mut img: RgbImage = ImageBuffer::new(dims.0 as u32, dims.1 as u32);
    let starting_triangle = max_centered_eq_triangle(dims);

    drawing::draw_polygon_mut(&mut img, &starting_triangle.points, base_color);

    triangle_vec.push(starting_triangle);

    loop {
        let mut sub_triangles: Vec<Triangle> = Vec::new();

        for tri in &triangle_vec {
            let (center_triangle, mut corner_triangles) = subdivide_triangle(tri);

            drawing::draw_polygon_mut(&mut img, &center_triangle.points, empty_color);
            sub_triangles.append(&mut corner_triangles);
        }

        if let Some(tri) = sub_triangles.first() {
            if area(&tri) > min_area {
                triangle_vec = sub_triangles;
            } else {
                break;
            }
        };
    }

    img.save("fractal.png").unwrap();
}
