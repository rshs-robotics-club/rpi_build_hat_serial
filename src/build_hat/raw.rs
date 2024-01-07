use std::ops::DerefMut;

use rppal::uart::Uart;

use super::*;
/// Write data to the build hat.
pub async fn write(serial: &mut impl DerefMut<Target = Uart>, data: &[u8]) -> Result<()> {
    // let mut serial = UART_SERIAL.lock().await;
    let _ = serial.write(data)?;
    // Ok(serial.write(data)?)
    Ok(())
}
pub async fn write_and_skip(serial: &mut impl DerefMut<Target = Uart>, data: &[u8]) -> Result<()> {
    let written_len = write(serial, data).await?;
    let _ = skip_prompt(serial, data).await?;
    Ok(())
}
/// Read a line as a string
pub async fn read_line(serial: &mut impl DerefMut<Target = Uart>) -> Result<String> {
    let mut complete_line = String::new();
    loop {
        let mut buffer = [0u8; 64];
        match serial.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // do nothing
                    continue;
                } else {
                    let decoded_str = CStr::from_bytes_until_nul(&buffer).expect("CStr decode fail");
                    // print!("{:?}", decoded_str);
                    complete_line.push_str(decoded_str.to_str().expect("Cstr to thing convert fail"));
                    if complete_line.ends_with("\r\n") {
                        // println!("{complete_line}");
                        return Ok(complete_line);
                    }
                    // println!("{:#?}", s);
                }
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
}
// skip one line.
pub async fn skip_line(serial: &mut impl DerefMut<Target = Uart>) -> Result<()> {
    let _ = read_line(serial).await?;
    Ok(())
}
pub async fn skip_content(serial: &mut impl DerefMut<Target = Uart>, content: &[u8]) -> Result<()> {
    let mut len_left = content.len();
    loop {
        let mut buffer = [0u8;64];
        match serial.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // println!("it was me");
                    continue;
                }
                len_left -= bytes_read;
                let read = &buffer[0..bytes_read];
                let mut complete = String::new();
                read.iter().for_each(|a| {
                    let part: Vec<u8> = std::ascii::escape_default(*a).collect();
                    complete.push_str(std::str::from_utf8(&part).unwrap());
            });
                // print!("{}",complete);
                // println!("left {len_left}");
                if len_left == 0 {
                    println!("ready to break");
                    break;
                }
            }
            Err(e) => {
                panic!("whoopsies {:#?}", e);
            }
        }
    }
    Ok(())
}
pub async fn skip_prompt(serial: &mut impl DerefMut<Target = Uart>, content: &[u8]) -> Result<()> {
    skip_content(serial, "BHBL> ".as_bytes()).await?;
    skip_content(serial, content).await?;
    Ok(())
}

// pub async fn send_help() -> Result<()> {
//     write("help\r".as_bytes())
// }
// pub async fn send_version() -> Result<()> {
//     write("version\r".as_bytes())
// }
macro_rules! send_command {
    ($name:ident, $command:literal) => {
        paste::item! {
            pub async fn [< send_ $name >] (serial: &mut impl DerefMut<Target=Uart>) -> Result<()> {
                write(serial, $command.as_bytes()).await.map(|_| ())
            }
        }
    };
}
send_command!(version, "version\r");
send_command!(help, "help\r");
send_command!(clear, "clear\r");
send_command!(verify, "verify\r");
send_command!(reboot, "reboot\r");
const STX: u8 = 0x02;
const ETX: u8 = 0x03;
pub async fn send_load(
    serial: &mut impl DerefMut<Target = Uart>,
    length: &str,
    checksum: u32,
    data: &[u8],
) {
    let mut first_str = format!("load {length} {checksum}\r").as_bytes().to_vec();
    // first_str.append(&mut checksum.to_vec());
    // first_str.push(b'\r');
    let _ = write(serial, &first_str).await.unwrap();
    // let _ = skip_content(serial, &first_str).await.unwrap();
    // let mut send_data = vec![STX];
    // send_data.append(&mut data.to_vec());
    // send_data.push(ETX);
    // send_data.push(b'\r');
    // let _ = write(serial, &send_data).await.unwrap();
    // let _ = skip_prompt(serial, &send_data).await.unwrap();
}
pub async fn send_signature(serial: &mut impl DerefMut<Target = Uart>, length: &str, data: &[u8]) {
    let first_str = format!("signature {length}\r");
    let _ = write(serial, first_str.as_bytes()).await;
    let _ = skip_line(serial).await;
    let mut send_data = vec![STX];
    send_data.append(&mut data.to_vec());
    send_data.push(ETX);
    let _ = write(serial, &send_data).await;
    let _ = skip_line(serial).await;
}
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
