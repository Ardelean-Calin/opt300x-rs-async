//! This is a platform agnostic Rust driver for the OPT300x ambient light
//! sensors using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! The OPT3001 is a sensor that measures the intensity of visible light.
//! The spectral response of the sensor tightly matches the photopic
//! response of the human eye and includes significant infrared rejection.
//!
//! The OPT3001 is a single-chip lux meter, measuring the intensity of
//! light as visible by the human eye. The precision spectral response and
//! strong IR rejection of the device enables the OPT3001 to accurately
//! meter the intensity of light as seen by the human eye regardless of
//! light source. The strong IR rejection also aids in maintaining high
//! accuracy when industrial design calls for mounting the sensor under
//! dark glass for aesthetics. The OPT3001 is designed for systems that
//! create light-based experiences for humans, and an ideal preferred
//! replacement for photodiodes, photoresistors, or other ambient light
//! sensors with less human eye matching and IR rejection.
//!
//! Measurements can be made from 0.01 lux up to 83k lux without manually
//! selecting full-scale ranges by using the built-in, full-scale setting
//! feature. This capability allows light measurement over a 23-bit
//! effective dynamic range.
//!
//! The digital operation is flexible for system integration. Measurements
//! can be either continuous or single-shot. The control and interrupt
//! system features autonomous operation, allowing the processor to sleep
//! while the sensor searches for appropriate wake-up events to report via
//! the interrupt pin. The digital output is reported over an I2C- and
//! SMBus-compatible, two-wire serial interface.
//!
//! The low power consumption and low power-supply voltage capability of the
//! OPT3001 enhance the battery life of battery-powered systems.
//!
//! Datasheets:
//! - [OPT3001](https://www.ti.com/lit/ds/symlink/opt3001.pdf)
//! - [OPT3002](https://www.ti.com/lit/ds/symlink/opt3002.pdf)
//! - [OPT3004](https://www.ti.com/lit/ds/symlink/opt3004.pdf)
//! - [OPT3006](https://www.ti.com/lit/ds/symlink/opt3006.pdf)
//! - [OPT3007](https://www.ti.com/lit/ds/symlink/opt3007.pdf)
//!
//! Application Guide:
//! - [OPT3001 ALS Application Guide](https://www.ti.com/lit/an/sbea002a/sbea002a.pdf)

#![deny(unsafe_code, missing_docs)]
#![no_std]

use core::marker::PhantomData;
extern crate embedded_hal as hal;

/// Errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus communication error
    I2C(E),
    /// Invalid input data provided
    InvalidInputData,
}

/// Error type for mode changes.
///
/// This allows to retrieve the unchanged device in case of an error.
pub enum ModeChangeError<E, DEV> {
    /// I²C bus error while changing mode.
    ///
    /// `E` is the error that happened.
    /// `DEV` is the device with the mode unchanged.
    I2C(E, DEV),
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Config {
    bits: u16,
}

impl Config {
    fn with_high(self, mask: u16) -> Self {
        Config {
            bits: self.bits | mask,
        }
    }
    fn with_low(self, mask: u16) -> Self {
        Config {
            bits: self.bits & !mask,
        }
    }
}

/// IC markers
#[doc(hidden)]
pub mod ic {
    /// Used for OPT3001 devices
    pub struct Opt3001(());
}

/// Mode marker
pub mod mode {
    /// One shot mode
    pub struct OneShot(());
    /// Continuous measurement mode
    pub struct Continuous(());
}

/// OPT300x device driver
#[derive(Debug)]
pub struct Opt300x<I2C, IC, MODE> {
    i2c: I2C,
    address: u8,
    config: Config,
    _ic: PhantomData<IC>,
    _mode: PhantomData<MODE>,
}

/// Possible slave addresses
#[derive(Debug, Clone, Copy)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit values for A1 and A0
    Alternative(bool, bool),
}

/// Fault count
///
/// Number of consecutive fault events necessary to trigger interrupt.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FaultCount {
    /// One (default)
    One,
    /// Two
    Two,
    /// Four
    Four,
    /// Eight
    Eight,
}

/// Interrupt pin polarity (active state)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterruptPinPolarity {
    /// Active low (default)
    Low,
    /// Active high
    High,
}

/// Lux range
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LuxRange {
    /// Manual [0-11]
    Manual(u8),
    /// Automatic selection (default)
    Auto,
}

/// Integration time
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntegrationTime {
    /// 100 ms
    Ms100,
    /// 800 ms
    Ms800,
}

mod device_impl;
mod slave_addr;

mod private {
    use super::{ic, mode};
    pub trait Sealed {}

    impl Sealed for ic::Opt3001 {}
    impl Sealed for mode::OneShot {}
    impl Sealed for mode::Continuous {}
}
