use crate::motor_wrap::{self, Motor};

pub struct Omni{
    pub motor_a: Motor,
    pub motor_b: Motor,
    pub motor_c: Motor,
    pub motor_d: Motor,
}
impl Omni{
    /// creates a new omni object.
    /// 
    /// # Parameters
    /// * limit: Limit to how fast the motors can go. Usually 1.0 (100%)
    pub async fn new(limit: f32) -> Self {
        let a = Motor::new(crate::raw::firmware::Port::A, limit).await;
        let b = Motor::new(crate::raw::firmware::Port::B, limit).await;
        let c = Motor::new(crate::raw::firmware::Port::C, limit).await;
        let d = Motor::new(crate::raw::firmware::Port::D, limit).await;
        Self { motor_a: a, motor_b: b, motor_c: c, motor_d: d }
    }
    pub async fn run_raw(&self, a_speed: i8, b_speed: i8, c_speed: i8, d_speed: i8){
        self.motor_a.run(a_speed).await;
        self.motor_b.run(b_speed).await;
        self.motor_c.run(c_speed).await;
        self.motor_d.run(d_speed).await;
    }
}