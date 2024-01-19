use std::fmt::format;

use super::*;

create_send_commands! {
    /// Display a list of commands available
    {help, "help\r"},
    /// list connected devices.
    {list, "list\r"},
    /// report main power input voltage.
    {vin, "vin\r"},
    /// clear latched motor fault conditions.
    {clear_faults, "clear_faults\r"},
    /// disable motor driver.
    {coast, "coast\r"},
    /// set current port to direct PWM mode (default).
    {pwm, "pwm\r"},
    /// same as pwm ; set 0.
    {off, "off\r"},
    /// same as pwm ; set 0.
    {on, "on\r"},
    /// print version string.
    {version, "version\r"},
    /// dump firmware signature.
    {signature, "signature\r"},
    /// select stop
    {select_stop, "select\r"}
}

/// Represents a port on the build hat.
#[derive(Clone)]
#[repr(u8)]
pub enum Port {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}
/// select a port (default 0).
pub async fn send_port(serial: &mut impl DerefMut<Target = Uart>, port: Port) -> Result<()> {
    let s = format!("port {}\r", port as u8);
    let _ = write(serial, s.as_bytes()).await?;
    Ok(())
}
/// Represents the orange and green led's mode on the build hat.
#[repr(i8)]
pub enum LedMode {
    /// Change colors based on the input voltage (default).
    MonitorVin = -1,
    Off = 0,
    Orange = 1,
    Green = 2,
    Both = 3,
}
/// set LED function
pub async fn send_ledmode(
    serial: &mut impl DerefMut<Target = Uart>,
    mode: LedMode,
) -> Result<()> {
    let s = format!("ledmode {}\r", mode as i8);
    let _ = write_and_skip(serial, s.as_bytes()).await?;
    Ok(())
}
// todo: implement pid
// todo: implement pid_diff
/// configure constant set point for current port.
pub async fn send_set_point(
    serial: &mut impl DerefMut<Target = Uart>,
    setpoint: f32,
) -> Result<(), SerialError> {
    let s = format!("set {}\r", setpoint);
    let _ = write_and_skip(serial, s.as_bytes()).await?;
    Ok(())
}
/// Parameters for [`send_set_waveparams`].
/// Useful for creating a varying set point
#[non_exhaustive]
pub enum WaveParams {
    Square {
        min: f32,
        max: f32,
        period: f32,
        phase: f32,
    },
    Sine {
        min: f32,
        max: f32,
        period: f32,
        phase: f32,
    },
    Triangle {
        min: f32,
        max: f32,
        period: f32,
        phase: f32,
    },
    Pulse {
        during: f32,
        after: f32,
        length: f32,
    },
    Ramp {
        from: f32,
        to: f32,
        duration: f32,
    },
    // to implement: var
}
impl ToString for WaveParams {
    fn to_string(&self) -> String {
        match self {
            Self::Square {
                min,
                max,
                period,
                phase,
            } => format!("square {min} {max} {period} {phase}"),
            Self::Sine {
                min,
                max,
                period,
                phase,
            } => format!("sine {min} {max} {period} {phase}"),
            Self::Triangle {
                min,
                max,
                period,
                phase,
            } => format!("triangle {min} {max} {period} {phase}"),
            Self::Pulse {
                during,
                after,
                length,
            } => format!("pulse {during} {after} {length} 0"),
            Self::Ramp { from, to, duration } => format!("ramp {from} {to} {duration} 0"),
            // to implement: var
        }
    }
}
/// configure varying set point for current port.
pub async fn send_set_waveparams(
    serial: &mut impl DerefMut<Target = Uart>,
    params: WaveParams,
) -> Result<()> {
    let s = format!("set {}\r", params.to_string());
    write_and_skip(serial, s.as_bytes()).await?;
    Ok(())
}
/// Represents parameters for PWM driver. Refer to [`send_pwmparams`]
pub struct PwmParams {
    pub pwmthresh: f32,
    pub minpwm: f32,
}
impl ToString for PwmParams {
    fn to_string(&self) -> String {
        format!("{} {}", self.pwmthresh, self.minpwm)
    }
}

/// configure parameters for PWM driver.
pub async fn send_pwmparams(
    serial: &mut impl DerefMut<Target = Uart>,
    pwmparams: PwmParams,
) -> Result<()> {
    let s = format!("pwmparams {}\r", pwmparams.to_string());
    write_and_skip(serial, s.as_bytes()).await?;
    Ok(())
}
/// set PWM output drive limit for all ports (default 0.1).
pub async fn send_plimit(
    serial: &mut impl DerefMut<Target = Uart>,
    limit: f32,
) -> Result<(), SerialError> {
    write_and_skip(serial, format!("plimit {limit}\r").as_bytes()).await
}
/// set PWM output drive limit for current port.
pub async fn send_port_plimit(
    serial: &mut impl DerefMut<Target = Uart>,
    limit: f32,
) -> Result<(), SerialError> {
    write_and_skip(serial, format!("port_plimit {limit}\r").as_bytes()).await
}

// todo: send_select_var
/// send a SELECT message to select a mode and output all its data in raw hex.
pub async fn select_mode(
    serial: &mut impl DerefMut<Target = Uart>,
    mode: u8,
) -> Result<(), SerialError> {
    write_and_skip(serial, format!("select {mode}\r").as_bytes()).await
}
// todo: select
/// as [`select_mode'] but only report one data packet.
pub async fn selonce_mode(
    serial: &mut impl DerefMut<Target = Uart>,
    mode: u8,
) -> Result<(), SerialError> {
    write_and_skip(serial, format!("selonce {mode}\r").as_bytes()).await
}
// todo: selrate
// todo: combi
// todo: combi
// todo: write1
// todo: write2
/// enable/disable echo and prompt on command port.
pub async fn send_echo(
    serial: &mut impl DerefMut<Target = Uart>,
    enable_echo: bool,
) -> Result<(), SerialError> {
    write_and_skip(serial, format!("echo {}", enable_echo as u8).as_bytes()).await
}
// todo: debug
