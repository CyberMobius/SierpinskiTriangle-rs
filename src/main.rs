use image;
use imageproc;

use image::{ImageBuffer, RgbImage};
use imageproc::drawing;
use imageproc::point::Point;

struct Triangle {
    points: [Point<i32>; 3],
}

fn area(triangle: &Triangle) -> i32 {
    let v = triangle.points;
    let (a, b, c) = (v[0], v[1], v[2]);

    return (a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y)) / 2;
}

fn midpoint(point_1: &Point<i32>, point_2: &Point<i32>) -> Point<i32> {
    let mid_x = (point_1.x + point_2.x) / 2;
    let mid_y = (point_1.y + point_2.y) / 2;

    Point { x: mid_x, y: mid_y }
}

fn split_triangle(triangle: &Triangle) -> Vec<Triangle> {
    let p = triangle.points;
    let sub_triangle = |index_list: &[usize; 3]| {
        let head = &p[index_list[0]];

        let left = &p[index_list[1]];
        let left = midpoint(head, left);

        let right = &p[index_list[2]];
        let right = midpoint(head, right);

        return Triangle {
            points: [*head, left, right],
        };
    };

    [[0, 1, 2], [1, 2, 0], [2, 0, 1]]
        .iter()
        .map(|indices| sub_triangle(indices))
        .collect()
}

fn center_triangle(triangle: &Triangle) -> Triangle{
    let p = triangle.points;
    return Triangle {
        points: [midpoint(&p[0], &p[1]), midpoint(&p[1], &p[2]), midpoint(&p[2], &p[0])]
    };
}

fn main() {
    let dims: (i32, i32) = (1024, 1024);

    let top_center = Point {
        x: (dims.0 / 2),
        y: 0,
    };
    let bottom_right = Point {
        x: (dims.0 - 1),
        y: (dims.1 - 1),
    };
    let bottom_left = Point {
        x: 0,
        y: (dims.1 - 1),
    };

    let mut triangle_vec: Vec<Triangle> = Vec::new();
    let starting_triangle = Triangle {
        points: [top_center, bottom_right, bottom_left],
    };

    let mut img: RgbImage = ImageBuffer::new(dims.0 as u32, dims.1 as u32);
    
    let white = image::Rgb([255, 255, 255]);
    let black = image::Rgb([0, 0, 0]);

    drawing::draw_polygon_mut(&mut img, &starting_triangle.points, white);

    triangle_vec.push(starting_triangle);

    loop {
        println!("{}", triangle_vec.len());
        let mut sub_triangles = Vec::new();
        for tri in &triangle_vec {
            drawing::draw_polygon_mut(&mut img, &center_triangle(&tri).points, black);
            sub_triangles.append(&mut split_triangle(&tri));
        }

        match sub_triangles.first() {
            Some(tri) => {
                if area(&tri) > 24{
                    triangle_vec = sub_triangles;
                }
                else {
                    break;
                }
            }
            _ => {}
        }
    }

    img.save("fractal.png").unwrap();
}
