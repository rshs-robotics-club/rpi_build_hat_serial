use std::hash::BuildHasher;

use pollster;
use rpi_build_hat_serial::build_hat;
use rpi_build_hat_serial::build_hat::raw::checksum;
use rpi_build_hat_serial::*;

fn transform_u32_to_array_of_u8(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}
fn main() {
    let _ = pollster::block_on(async move {
        let mut serial = UART_SERIAL.lock().await;
        let firmware_len = FIRMWARE.len();
        println!("will be sending firmware with len {firmware_len}");
        build_hat::raw::send_load(
            &mut serial,
            firmware_len.to_string().as_str(),
            // &transform_u32_to_array_of_u8(checksum(&FIRMWARE)),
            checksum(&FIRMWARE),
            &FIRMWARE,
        ).await;
        // build_hat::raw::send_help(&mut serial).await;
        println!("enter read loop");
        loop {
            let s = build_hat::raw::read_line(&mut serial).await;
            if let Ok(s) = s {
                // println!("ok well {s}");
                print!("ok well {s}");
            }
        }
    });
}
