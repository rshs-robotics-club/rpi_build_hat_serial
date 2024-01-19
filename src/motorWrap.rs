use std::clone;

use crate::{raw::*, UART_SERIAL};
use rppal::uart::Uart;
use crate::raw::firmware::{send_port, send_plimit, send_pwm};

use self::firmware::{send_set_point, Port};
pub struct Motor{
pub speed: u8,
pub port: Port,
pub limit: f32,
}
impl Motor{
pub async fn new(motor_port: Port, limit: f32) -> Self {
    let mut serial = UART_SERIAL.lock().await;
    send_plimit(&mut serial, limit).await;
    Self { speed: 0, port: motor_port, limit: limit }
}
pub async fn run(&self, speed: i8){
    if (speed > 100 || speed < -100){
        
    }
    let mut serial = UART_SERIAL.lock().await;
    let _ = send_port(&mut serial, self.port.clone()).await;
    let _ = send_pwm(&mut serial).await;
    let _ = send_set_point(&mut serial, speed as f32 / 100.0).await;
}

}