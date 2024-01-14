/// Functions for communicating with the bootloader.
use std::{
    ffi::{CStr, CString},
    path::Component,
    str::FromStr,
};
// #[macro_use]
use super::*;

use crate::UART_SERIAL;
use anyhow::Result;
use iso8601::DateTime;

// pub async unsafe fn skip_verify_success(serial: &mut impl DerefMut<Target = Uart>) -> Result<()> {
//     let _ = skip_line(serial).await?; // BHBL> verify\n
//     let _ = skip_line(serial).await?; // Verifying Image...
//     let _ = skip_line(serial).await?; // Verying Image lemgth=....
//     let _ = skip_line(serial).await?; // SHA256:
//     let _ = skip_line(serial).await?; // .......
//     let _ = skip_line(serial).await?; // Public Key:
//     let _ = skip_line(serial).await?; // .........
//     let _ = skip_line(serial).await?; // Image verified OK
//     Ok(())
// }

// pub async fn send_help() -> Result<()> {
//     write("help\r".as_bytes())
// }
// pub async fn send_version() -> Result<()> {
//     write("version\r".as_bytes())
// }
create_send_commands! {
    {version, "version\r"},
    {help, "help\r"},
    {clear, "clear\r"},
    {verify, "verify\r"},
    {reboot, "reboot\r"}
}
const STX: u8 = 0x02;
const ETX: u8 = 0x03;
pub async unsafe fn send_load(
    serial: &mut impl DerefMut<Target = Uart>,
    length: &str,
    checksum: u32,
    data: &[u8],
) {
    let mut first_str = format!("load {length} {checksum}\r").as_bytes().to_vec();
    // first_str.append(&mut checksum.to_vec());
    // first_str.push(b'\r');
    let _ = write_and_skip(serial, &first_str).await.unwrap();
    // let _ = skip_content(serial, &first_str).await.unwrap();
    let mut send_data = vec![STX];
    send_data.append(&mut data.to_vec());
    send_data.push(ETX);
    send_data.push(b'\r');
    let _ = write(serial, &send_data).await.unwrap(); // todo: check why skip isn't required here
}
pub async unsafe fn send_signature(
    serial: &mut impl DerefMut<Target = Uart>,
    length: &str,
    data: &[u8],
) {
    let first_str = format!("signature {length}\r");
    let _ = write_and_skip(serial, first_str.as_bytes()).await;
    // let _ = skip_line(serial).await;
    let mut send_data = vec![STX];
    send_data.append(&mut data.to_vec());
    send_data.push(ETX);
    send_data.push(b'\r');
    let _ = write_and_skip(serial, &send_data).await;
    // let _ = skip_line_ending(serial).await;
    // let _ = skip_prompt(serial).await;
    // let _ = skip_line_ending(serial).await;

    // let _ = skip_line(serial).await;
}
/// Calculate the checksum required for uploading firmware.
pub fn checksum(data: &[u8]) -> u32 {
    let mut u: u32 = 1;
    for &byte in data {
        if (u & 0x80000000) != 0 {
            u = (u << 1) ^ 0x1d872b41;
        } else {
            u <<= 1;
        }
        u = (u ^ u32::from(byte)) & 0xFFFFFFFF;
    }
    u
}
