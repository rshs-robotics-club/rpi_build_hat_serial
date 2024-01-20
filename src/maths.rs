use std::f32::consts::PI;

/// a 2-dimensional vector.
pub struct Vector2{
    x: f32,
    y: f32,
}

/// calculates rotation matrix.
///
/// Rotates a point counter clockwise about the origin.
/// 
/// # Parameters
/// * x1: the x value of the starting point.
/// * x2: the y value of the starting point.
/// * angle: the angle in which the point rotates. (degrees)
/// 
/// ### Return type: Vector2
fn find_rotated_point(x1: f32, y1: f32, angle: f32) -> Vector2 {
    let x2: f32 = (x1 * ((angle*PI/180.0).cos())) + (y1 * (-(angle*PI/180.0).sin()));
    let y2: f32 = (x1 * ((angle*PI/180.0).sin()))+ (y1 * (angle*PI/180.0).cos());
    Vector2 {x: x2, y: y2}
}
