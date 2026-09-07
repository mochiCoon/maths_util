use crate::arithmetic::{max, min};

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn inverse_lerp(a: f32, b: f32, value: f32) -> f32 {
    (value - a) / (b - a)
}

pub fn remap(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    lerp(out_min, out_max, inverse_lerp(in_min, in_max, value))
}

pub fn smoothstep(in_min: f32, in_max: f32, value: f32) -> f32 {
    let x: f32 = max(0.0, min(1.0, (value-in_min)/(in_max-in_min)));
    x*x*(3.0 - 2.0*x)
}