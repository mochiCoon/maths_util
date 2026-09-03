pub mod arithmetic;
pub mod constants;
pub mod geometry;
pub mod interpolation;

#[cfg(test)]
mod tests {
    use crate::arithmetic::{abs, clamp, cube, max, min, pow, sign, sqrt, square};
    use crate::geometry::{
        area_of_circle, area_of_quadrilateral, area_of_triangle, circumference_of_circle,
        distance_2d, distance_3d, perimeter_of_quadrilateral,
    };
    use crate::interpolation::{inverse_lerp, lerp, remap};

    // ==================== GEOMETRY ====================

    #[test]
    fn test_area_of_circle() {
        let result = area_of_circle(5.0);
        assert!((result - 78.539816).abs() < 0.0001);
    }

    #[test]
    fn test_circumference_of_circle() {
        let result = circumference_of_circle(5.0);
        assert!((result - 31.415926).abs() < 0.0001);
    }

    #[test]
    fn test_area_of_quadrilateral() {
        assert!(area_of_quadrilateral(2.0, 2.0) == 4.0);
    }

    #[test]
    fn test_perimeter_of_quadrilateral() {
        assert!(perimeter_of_quadrilateral(2.0, 2.0) == 8.0)
    }

    #[test]
    fn test_area_of_triangle() {
        assert!(area_of_triangle(10.0, 6.0) == 30.0)
    }

    #[test]
    fn test_distance_2d() {
        let result = distance_2d(0.0, 0.0, 3.0, 4.0);

        assert!((result - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_distance_3d() {
        assert!(distance_3d(0.0, 0.0, 0.0, 3.0, 4.0, 12.0) == 13.0)
    }

    // ==================== ARITHMETIC ====================

    #[test]
    fn test_min() {
        assert_eq!(min(5.0, 10.0), 5.0);
        assert_eq!(min(10.0, 5.0), 5.0);
        assert_eq!(min(-5.0, 5.0), -5.0);
    }

    #[test]
    fn test_max() {
        assert_eq!(max(5.0, 10.0), 10.0);
        assert_eq!(max(10.0, 5.0), 10.0);
        assert_eq!(max(-5.0, 5.0), 5.0);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
        assert_eq!(clamp(-5.0, 0.0, 10.0), 0.0);
        assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);
    }

    #[test]
    fn test_abs() {
        assert_eq!(abs(5.0), 5.0);
        assert_eq!(abs(-5.0), 5.0);
        assert_eq!(abs(0.0), 0.0);
    }

    #[test]
    fn test_sign() {
        assert_eq!(sign(5.0), 1.0);
        assert_eq!(sign(-5.0), -1.0);
        assert_eq!(sign(0.0), 0.0);
    }

    #[test]
    fn test_square() {
        assert_eq!(square(5.0), 25.0);
        assert_eq!(square(-5.0), 25.0);
        assert_eq!(square(0.0), 0.0);
    }

    #[test]
    fn test_cube() {
        assert_eq!(cube(5.0), 125.0);
        assert_eq!(cube(-5.0), -125.0);
        assert_eq!(cube(0.0), 0.0);
    }

    #[test]
    fn test_pow() {
        assert_eq!(pow(2.0, 0), 1.0);
        assert_eq!(pow(2.0, 1), 2.0);
        assert_eq!(pow(2.0, 5), 32.0);
        assert_eq!(pow(5.0, 2), 25.0);
    }

    #[test]
    fn test_sqrt() {
        assert!((sqrt(0.0) - 0.0).abs() < 0.0001);
        assert!((sqrt(1.0) - 1.0).abs() < 0.0001);
        assert!((sqrt(4.0) - 2.0).abs() < 0.0001);
        assert!((sqrt(9.0) - 3.0).abs() < 0.0001);
        assert!((sqrt(25.0) - 5.0).abs() < 0.0001);
        assert!((sqrt(2.0) - std::f32::consts::SQRT_2).abs() < 0.0001);
    }

    #[test]
    #[should_panic]
    fn test_sqrt_negative() {
        sqrt(-1.0);
    }

    // ==================== INTERPOLATION ====================

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
        assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
    }

    #[test]
    fn test_inverse_lerp() {
        assert_eq!(inverse_lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(inverse_lerp(0.0, 10.0, 5.0), 0.5);
        assert_eq!(inverse_lerp(0.0, 10.0, 10.0), 1.0);
    }

    #[test]
    fn test_lerp_negative_values() {
        assert_eq!(lerp(-10.0, 10.0, 0.5), 0.0);
    }

    #[test]
    fn test_inverse_lerp_negative_values() {
        assert_eq!(inverse_lerp(-10.0, 10.0, 0.0), 0.5);
    }

    #[test]
    fn test_lerp_outside_range() {
        assert_eq!(lerp(0.0, 10.0, 2.0), 20.0);
    }

    #[test]
    fn test_inverse_lerp_outside_range() {
        assert_eq!(inverse_lerp(0.0, 10.0, 20.0), 2.0);
    }

    #[test]
    fn test_remap() {
        assert!((remap(50.0, 0.0, 100.0, 0.0, 1.0) - 0.5).abs() < 0.0001);
        assert!((remap(25.0, 0.0, 100.0, 0.0, 10.0) - 2.5).abs() < 0.0001);
        assert!((remap(5.0, 0.0, 10.0, 100.0, 200.0) - 150.0).abs() < 0.0001);
    }
}
