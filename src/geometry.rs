use crate::arithmetic::{pow, sqrt};
use crate::constants::PI;

pub fn area_of_circle(radius: f32) -> f32 {
    return PI * (radius * radius);
}

pub fn circumference_of_circle(radius: f32) -> f32 {
    2.0 * PI * radius
}

pub fn area_of_quadrilateral(width: f32, height: f32) -> f32 {
    width * height
}

pub fn perimeter_of_quadrilateral(width: f32, height: f32) -> f32 {
    2.0 * (width + height)
}

pub fn area_of_triangle(base: f32, height: f32) -> f32 {
    0.5 * base * height
}

pub fn distance_2d(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx = x2 - x1;
    let dy = y2 - y1;

    sqrt(pow(dx, 2) + pow(dy, 2))
}

pub fn distance_3d(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> f32 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let dz = z2 - z1;
    sqrt(pow(dx, 2) + pow(dy, 2) + pow(dz, 2))
}
