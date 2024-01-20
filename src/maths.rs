use std::f32::consts::PI;
pub struct Vector2{
    x: f32,
    y: f32,
}
pub fn find_rotated_point(x1: f32, y1: f32, degrees: f32) -> Vector2 {
    let x2: f32 = ((x1 * (degrees * (3.1415926/180.0))) as f32).cos() + -((y1 * (degrees * (PI/180.0))) as f32).sin();
    let y2: f32 = ((x1 * (degrees * (3.1415926/180.0))) as f32).sin() + ((y1 * (degrees * (PI/180.0))) as f32).cos();
    Vector2 { x: x2, y: y2 }
}
#[test]
fn check(){
    let value: Vector2 = find_rotated_point(0.0, 1.0, 90.0);
    println!("the test results are: {} and {}", value.x, value.y);
}