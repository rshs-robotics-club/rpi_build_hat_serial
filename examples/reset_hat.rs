use std::hash::BuildHasher;

use pollster;
use rpi_build_hat_serial::raw::bootloader::*;
use rpi_build_hat_serial::raw::*;
use rpi_build_hat_serial::*;
fn main() {
    unsafe {
        let _ = pollster::block_on(async move {
            let mut serial = UART_SERIAL.lock().await;
            reset_hat(&mut serial).await;
            // build_hat::raw::send_help(&mut serial).await;
            println!("enter read loop");
            loop {
                let s = read_line(&mut serial).await;
                if let Ok(s) = s {
                    let s = s.strip_suffix("\r\n").unwrap_or(&s);
                    // println!("ok well {s}");
                    println!("-->{s}<--");
                }
            }
        });
    }
}
