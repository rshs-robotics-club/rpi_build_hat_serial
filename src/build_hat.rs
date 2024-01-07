use std::{
    ffi::{CStr, CString},
    path::Component,
    str::FromStr,
};

use crate::{HatResponse, UART_SERIAL};
use anyhow::Result;
use iso8601::DateTime;
pub mod raw;

// pub fn string_to_response(s: String) -> Result<HatResponse> {
//     if s.starts_with("BuildHAT Bootloader version ") {
//         let version_datetime = s
//             .strip_prefix("BuildHAT Bootloader version ")
//             .unwrap()
//             .strip_suffix("\r\n")
//             .unwrap();
//         let mut split = version_datetime.split(' ');
//         let version: String = split.next().unwrap().into();
//         let datetime = DateTime::from_str(split.next().unwrap().into()).unwrap();
//         return Ok(HatResponse::BootLoaderVersion(version, datetime));
//     }
//     todo!()
// }
// pub mod bootloader {
//     use super::raw::*;
//     use crate::build_hat::string_to_response;
//     use crate::*;
//     pub async fn get_bootloader_version() -> Result<HatResponse> {
//         let _ = write("version\r".as_bytes()).await;
//         let _ = skip_read_line().await?;
//         let line = read_line_raw().await?;
//         let response = string_to_response(line)?;
//         Ok(response)
//     }
// }
