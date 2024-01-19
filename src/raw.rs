//! Functions for directly communicating with the build hat.

use std::{ops::DerefMut, thread::sleep};

use rppal::{gpio::Gpio, uart::Uart};
pub mod bootloader;
pub mod firmware;
use super::*;
use std::ffi::CStr;

/// Represents an error within the raw read/write operations.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum SerialError {
    #[error("Failed to write data")]
    WriteFailed(rppal::uart::Error),
    #[error("Failed to read data")]
    ReadFailed(rppal::uart::Error),
    #[error(
        "Could not decode buffer data as C-style string because an ending NULL byte was missing"
    )]
    NullByteMissing(#[from] std::ffi::FromBytesUntilNulError),
    #[error("Could not convert C-style string to UTF-8 format")]
    Utf8ConversionFailed(#[from] std::str::Utf8Error),
    #[error("Could not initialise gpio pins")]
    GpioInitFailed,
}
/// Reset the build hat.
pub async fn reset_hat(
    serial: &mut impl DerefMut<Target = Uart>,
) -> Result<(), SerialError> {
    let gpio = Gpio::new().map_err(|_| SerialError::GpioInitFailed)?;
    let mut reset_pin = gpio
        .get(4)
        .map_err(|_| SerialError::GpioInitFailed)?
        .into_output();
    let mut boot_pin = gpio
        .get(0)
        .map_err(|_| SerialError::GpioInitFailed)?
        .into_output();
    boot_pin.set_low();
    reset_pin.set_low();
    sleep(Duration::from_millis(10));
    reset_pin.set_high();
    sleep(Duration::from_millis(10));
    boot_pin.set_low();
    reset_pin.set_low();
    sleep(Duration::from_millis(500));
    Ok(())
}
/// Write data to the build hat.
///
/// Consider using [`write_and_skip`] instead.
pub async fn write(
    serial: &mut impl DerefMut<Target = Uart>,
    data: &[u8],
) -> Result<usize, SerialError> {
    serial.write(data).map_err(|e| SerialError::WriteFailed(e))
}
/// Write data to the build hat and skip the same line.
///
/// Rppal's Uart echoes the data you write. This function skips that data so that it doesn't get clogged up
/// in other read functions.
pub async fn write_and_skip(
    serial: &mut impl DerefMut<Target = Uart>,
    data: &[u8],
) -> Result<(), SerialError> {
    let written_len = write(serial, data).await?;
    let _ = skip_content(serial, data).await?;
    Ok(())
}
/// Read a line as a string. This function will block forever until a line is read.
pub async fn read_line(
    serial: &mut impl DerefMut<Target = Uart>,
) -> Result<String, SerialError> {
    let mut complete_line = String::new();
    loop {
        let mut buffer = [0u8; 64];
        match serial.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // do nothing
                    continue;
                } else {
                    let decoded_str = CStr::from_bytes_until_nul(&buffer)?;
                    complete_line.push_str(decoded_str.to_str()?);
                    if complete_line.ends_with("\r\n") {
                        return Ok(complete_line);
                    }
                }
            }
            Err(e) => {
                return Err(SerialError::ReadFailed(e));
            }
        }
    }
}
/// skip one line.
pub async fn skip_line(
    serial: &mut impl DerefMut<Target = Uart>,
) -> Result<(), SerialError> {
    read_line(serial).await.map(|_| ())
}
/// Skip content such as the Uart echo after a write is performed.
pub async fn skip_content(
    serial: &mut impl DerefMut<Target = Uart>,
    content: &[u8],
) -> Result<(), SerialError> {
    let mut len_left = content.len();
    loop {
        let mut buffer = [0u8; 64];
        match serial.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    continue;
                }
                len_left -= bytes_read;
                let read = &buffer[0..bytes_read];
                let mut complete = String::new();
                for c in read {
                    let part: Vec<u8> = std::ascii::escape_default(*c).collect();
                    let str_conversion = std::str::from_utf8(&part);
                    if let Ok(str) = str_conversion {
                        complete.push_str(str);
                    } else if let Err(e) = str_conversion {
                        return Err(SerialError::Utf8ConversionFailed(e));
                    }
                }
                if len_left == 0 {
                    // println!("ready to break");
                    break;
                }
            }
            Err(e) => {
                return Err(SerialError::ReadFailed(e));
            }
        }
    }
    Ok(())
}
/// Skip the text "BHBL> ". This piece of text appears on the bootloader terminal(?).
pub async fn skip_prompt(serial: &mut impl DerefMut<Target = Uart>) -> Result<()> {
    skip_content(serial, "BHBL> ".as_bytes()).await?;
    // skip_content(serial, content).await?;
    Ok(())
}
/// Skip a `\r\n` line ending. (CRLF)
pub async fn skip_line_ending(serial: &mut impl DerefMut<Target = Uart>) -> Result<()> {
    skip_content(serial, "\r\n".as_bytes()).await?;
    Ok(())
}
/// Create a function named `send_{ident}` which sends a string (second parameter)
macro_rules! send_command {
    ($(#[$meta:meta])*
    $name:ident, $command:literal) => {
        paste::item! {
            $(#[$meta])*
            pub async fn [< send_ $name >] (serial: &mut impl DerefMut<Target=Uart>) -> Result<(), SerialError> {
                write(serial, $command.as_bytes()).await.map(|_| ())
            }
        }
    };
}
pub(crate) use send_command;
macro_rules! create_send_commands {
    (
        $(
            $(#[$meta:meta])*
            {$name:ident, $command:literal}
        ),*
    ) => {
        $(
            paste::item! {
                $(#[$meta])*
                pub async fn [< send_ $name >] (serial: &mut impl DerefMut<Target=Uart>) -> Result<(), SerialError> {
                    write(serial, $command.as_bytes()).await.map(|_| ());
                    Ok(())
                }
            }
        )*
    }
}
pub(crate) use create_send_commands;
