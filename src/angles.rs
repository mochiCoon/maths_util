use crate::constants;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * (constants::PI / 180.0)
}

pub fn radians_to_degrees(radians: f32) ->  f32 {
    radians * (180.0 / constants::PI)
}