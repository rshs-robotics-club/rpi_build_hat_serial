#![allow(unused, dead_code)]
use std::time::{Duration, Instant};

use anyhow::{bail, Result};
use iso8601::DateTime;
use lazy_static::lazy_static;
use rppal::uart::Uart;
use thiserror::Error;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref UART_SERIAL: Mutex<Uart> = {
        let mut ser = Uart::with_path("/dev/serial0", 115200, rppal::uart::Parity::None, 8, 1)
            .expect("Serial port creation failed.");
        // ser.set_read_mode(0, Duration::ZERO);
        ser.set_read_mode(0, Duration::from_millis(5));
        ser.set_write_mode(true);
        Mutex::new(ser)
    };
    pub static ref FIRMWARE: &'static [u8] = { include_bytes!("firmware.bin") };
    pub static ref SIGNATURE: &'static [u8] = { include_bytes!("signature.bin") };
}

#[cfg(feature = "safe_abstraction")]
pub mod buildhat;
pub mod motor_wrap;
pub mod parser;
pub mod raw;
pub mod basic_sensor;