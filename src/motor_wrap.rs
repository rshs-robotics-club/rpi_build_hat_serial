use core::panic;
use std::clone;

use crate::{raw::*, UART_SERIAL};
use rppal::uart::Uart;
use crate::raw::firmware::{send_port, send_plimit, send_pwm};
use self::firmware::{send_set_point, Port};
#[derive(PartialEq)]
pub enum Direction{
    Clockwise,
    Anticlockwise,
}
pub struct Motor{
    pub speed: i8,
    pub port: Port,
    pub limit: f32,
    pub direction: Direction,
}
impl Motor{

    /// creates a new motor object.
    /// 
    /// # Parameters
    /// * motor_port: Port to which the motor is connected.
    /// * limit: Limit to how fast the motor can go. Usually 1.0 (100%).
    /// * motor_direction: which way the motor spins.
    pub async fn new(motor_port: Port, limit: f32, motor_direction: Direction) -> Self {
        let mut serial = UART_SERIAL.lock().await;
        send_plimit(&mut serial, limit).await;
        Self { speed: 0, port: motor_port, limit: limit, direction: motor_direction }
    }

    /// Rotates the motor at a given power
    /// 
    /// # Parameters
    /// * speed: the power (-100 to 100).
    pub async fn run(&self, speed: i8){
        if (speed > 100 || speed < -100){
            panic!("speed is over the limit!");
        }
        // only change the motor if the new speed is different to the previous speed
        if (speed != self.speed){
            let mut serial = UART_SERIAL.lock().await;
            let _ = send_port(&mut serial, self.port.clone()).await;
            let _ = send_pwm(&mut serial).await;
            if (self.direction == Direction::Clockwise){
                let _ = send_set_point(&mut serial, speed as f32 / 100.0).await;
            }
            else{
                let _ = send_set_point(&mut serial, (speed as f32 / 100.0) * -1.0 ).await;
            }
            
        }
    }



    pub async unsafe fn runf(&self, speed: f32){
        let raw_speed = speed / 100.0;
        if (speed > 1.0 || speed < -1.0){
            panic!("speed is over the limit");
        }
        // only change the motor if the new speed is different to the previous speed
        if (speed != (self.speed as f32)/100.0){
            let mut serial = UART_SERIAL.lock().await;
            let _ = send_port(&mut serial, self.port.clone()).await;
            let _ = send_pwm(&mut serial).await;
            let _ = send_set_point(&mut serial, raw_speed).await;
        }

    }

}