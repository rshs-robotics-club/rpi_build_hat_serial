//! This module is incomplete: do not use.
use either::Either;
use anyhow::Result;
use rppal::uart::Uart;
use std::{future::Future, time::Duration};
pub struct BootloaderHandle {}
pub struct FirmwareHandle {}

/// Get either a BootloaderHandle or a FirmwareHandle.
/// This function should only be called once.
pub async fn get_hat_handle() -> Result<(Either<BootloaderHandle, FirmwareHandle>, impl Future<Output = ()>)> {
    let mut socket = Uart::with_path("/dev/serial0", 115200, rppal::uart::Parity::None, 8, 1)
        .expect("Serial port creation failed.");
    socket.set_read_mode(0, Duration::ZERO);
    socket.set_write_mode(true).expect("write mode setting failed.");
    let update = async move {
        
    };
    Ok((Either::Left(BootloaderHandle {}), update))
}