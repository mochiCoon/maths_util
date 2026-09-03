pub fn min(a: f32, b: f32) -> f32 {
    if a <= b { a } else { b }
}
pub fn max(a: f32, b: f32) -> f32 {
    if a >= b { a } else { b }
}
pub fn clamp(value: f32, min_value: f32, max_value: f32) -> f32 {
    min(max(value, min_value), max_value)
}
pub fn abs(value: f32) -> f32 {
    if value < 0.0 { -value } else { value }
}
pub fn sign(value: f32) -> f32 {
    if value < 0.0 {
        -1.0
    } else if value > 0.0 {
        1.0
    } else {
        0.0
    }
}
pub fn square(value: f32) -> f32 {
    value * value
}
pub fn cube(value: f32) -> f32 {
    value * value * value
}
pub fn pow(base: f32, exponent: u32) -> f32 {
    let mut result = 1.0;

    for _ in 0..exponent {
        result *= base;
    }

    result
}

pub fn sqrt(x: f32) -> f32 {
    if x < 0.0 {
        panic!("Cannot calculate the square root of a negative number");
    }

    if x == 0.0 {
        return 0.0;
    }

    let mut guess = x;

    for _ in 0..20 {
        guess = (guess + x / guess) / 2.0;
    }

    guess
}
