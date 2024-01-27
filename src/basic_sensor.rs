use std::string;

use crate::{raw::{firmware::{self, select_mode, send_plimit, send_port, send_set_point, Port}, read_line}, UART_SERIAL};

struct BasicSensor{
    port: Port,
}
impl BasicSensor{
    async unsafe fn new(sensor_port: Port){
        Self {port: sensor_port}
    }
    async unsafe fn read(&self, mode: u8) -> string{
        let mut serial = UART_SERIAL.lock().await;
        let _ = send_port(&mut serial, self.port).await;
        let _ = send_plimit(&mut serial, 1.0).await;
        let _ = send_set_point(&mut serial, -1.0).await;
        let _ = select_mode(&mut serial, mode);
        let s = read_line(&mut serial).await.unwrap();
    }
}