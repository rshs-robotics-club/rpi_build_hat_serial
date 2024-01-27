use crate::motor_wrap::{self, Motor, Direction};
use crate::maths::{self, Vector2, find_rotated_point};
use std::cmp::max;
pub struct Omni{
    pub motor_a: Motor,
    pub motor_b: Motor,
    pub motor_c: Motor,
    pub motor_d: Motor,
    pub face_front: bool,
}
impl Omni{
    /// creates a new omni object.
    /// 
    /// # Parameters
    /// * limit: Limit to how fast the motors can go. Usually 1.0 (100%)
    pub async fn new(limit: f32, correction: bool) -> Self {
        let a = Motor::new(crate::raw::firmware::Port::A, limit, Direction::Anticlockwise).await;
        let b = Motor::new(crate::raw::firmware::Port::B, limit, Direction::Anticlockwise).await;
        let c = Motor::new(crate::raw::firmware::Port::C, limit, Direction::Anticlockwise).await;
        let d = Motor::new(crate::raw::firmware::Port::D, limit, Direction::Anticlockwise).await;
        Self { motor_a: a, motor_b: b, motor_c: c, motor_d: d, face_front: correction }
    }

    /// runs each motor individually.
    /// 
    /// this function shouldn't be used commonly as other functions are designed to be used
    /// in a more convinient way.
    /// 
    /// # Parameters
    /// * a_speed: speed of motor A.
    /// * b_speed: speed of motor B.
    /// * c_speed: speed of motor C.
    /// * d_speed: speed of motor D.
    pub async fn run_raw(&self, a_speed: i8, b_speed: i8, c_speed: i8, d_speed: i8){
        self.motor_a.run(a_speed).await;
        self.motor_b.run(b_speed).await;
        self.motor_c.run(c_speed).await;
        self.motor_d.run(d_speed).await;
    }

    /// runs the robot based on the angle provided.
    /// 
    /// # Parameters
    /// * top_speed: speed in which the robot aims to run at
    /// * robot_angle: the angle in which the robot runs towards. 0 degrees means directly 
    /// forwards, and it rotates anticlockwise.
    pub async fn run_angle(&self, top_speed: f32, robot_angle: f32, rotated_angle: f32) {
        let direction = find_rotated_point(0.0, 1.0, robot_angle);
        // this is the basic speed ratio. However, it does not allow the robot to rotate while it moves.
        let mut a = direction.x + direction.y;
        let mut b = direction.x - direction.y;
        let mut c = -direction.x - direction.y;
        let mut d = -direction.x + direction.y;
        if (self.face_front){
            let rotation_factor = (robot_angle + robot_angle) * 0.01;
            a = a - rotation_factor;
            b = b - rotation_factor;
            c = c - rotation_factor;
            d = d - rotation_factor;
            // the relative speed is done at this point, but the number should be adjusted
            // so that it matches the top speed given.
            let max_speed = [a, b, c, d].into_iter().reduce(f32::max).unwrap();
            let multiplier = top_speed/max_speed;
            let a_s: i8 = (a * multiplier) as i8;
            let b_s: i8 = (b * multiplier) as i8;
            let c_s: i8 = (c * multiplier) as i8;
            let d_s: i8 = (d * multiplier) as i8;
            self.run_raw(a_s, b_s, c_s, d_s).await;
        }
    }
}