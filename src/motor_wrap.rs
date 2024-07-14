use core::panic;
use std::clone;

use self::firmware::{send_set_point, Port};
use crate::raw::firmware::{send_plimit, send_port, send_pwm};
use crate::{raw::*, UART_SERIAL};
use firmware::select_mode;
use rppal::uart::Uart;
#[derive(PartialEq)]
pub enum Direction {
    Clockwise,
    Anticlockwise,
}
pub struct Motor {
    pub speed: f32,
    pub port: Port,
    pub limit: f32,
    pub direction: Direction,
}
impl Motor {
    /// creates a new motor object.
    ///
    /// # Parameters
    /// * motor_port: Port to which the motor is connected.
    /// * limit: Limit to how fast the motor can go. Usually 1.0 (100%).
    /// * motor_direction: which way the motor spins.
    pub async fn new(motor_port: Port, limit: f32, motor_direction: Direction) -> Self {
        let mut serial = UART_SERIAL.lock().await;
        send_plimit(&mut serial, limit).await;
        Self {
            speed: 0.0,
            port: motor_port,
            limit: limit,
            direction: motor_direction,
        }
    }
    /// Rotates the motor at a given power
    ///
    /// # Parameters
    /// * speed: the power (-100 to 100).
    pub async fn run_pwm(&mut self, mut speed: f32) {
        if (speed > 100.0) {
            speed = 100.0;
        }
        else if (speed < -100.0) {
            speed = -100.0;
        }
        // only change the motor if the new speed is different to the previous speed
        if (speed != self.speed) {
            let mut serial = UART_SERIAL.lock().await;
            let _ = send_port(&mut serial, self.port.clone()).await;
            let _ = send_pwm(&mut serial).await;
            if (self.direction == Direction::Clockwise) {
                let _ = send_set_point(&mut serial, speed / 100.0).await;
            } else {
                let _ = send_set_point(&mut serial, (speed / 100.0) * -1.0).await;
            }
            self.speed = speed;
        }
    }

    /// Rotates the motor at a given power
    ///
    /// # Parameters
    /// * speed: the power (-100 to 100).
    pub async fn run_pid(&mut self, mut speed: f32) {
        // only change the motor if the new speed is different to the previous speed
        if (speed > 100.0) {
            speed = 100.0;
        }
        else if (speed < -100.0) {
            speed = -100.0;
        }
        
        if (speed != self.speed) {
            let mut serial = UART_SERIAL.lock().await;
            let pid = format!("pid {} 0 0 s1 1 0 0.003 0.2 0 2 0.01 \r", self.port.clone() as u8);
            let _ = send_port(&mut serial, self.port.clone()).await;
            let _ = select_mode(&mut serial, 0).await;
            // selrate?
            let _ = write(&mut serial, pid.as_bytes()).await.unwrap();
            if (self.direction == Direction::Clockwise) {
                let _ = send_set_point(&mut serial, speed).await;
            } else {
                let _ = send_set_point(&mut serial, (speed) * -1.0).await;
            }

            self.speed = speed;
        }
    }

    pub async unsafe fn runf(&self, mut speed: f32) {
        let raw_speed = speed / 100.0;
        if (speed > 1.0) {
            speed = 1.0;
        }
        else if (speed < -1.0) {
            speed = -1.0;
        }
        // only change the motor if the new speed is different to the previous speed
        if (speed != (self.speed) / 100.0) {
            let mut serial = UART_SERIAL.lock().await;
            let _ = send_port(&mut serial, self.port.clone()).await;
            let _ = send_pwm(&mut serial).await;
            let _ = send_set_point(&mut serial, raw_speed).await;
        }
    }
}
