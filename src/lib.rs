#![no_std]

extern crate alloc;

mod axdriver_net;
mod igb;

pub use igb::*;

/// Vendor ID for Intel.
pub const INTEL_VEND: u16 = 0x8086;

/// Device ID for the 82576EB, used to identify the device from the PCI space.
pub const INTEL_82576: u16 = 0x10C9;

/// Device name
pub const DEVICE_NAME: &str = "igb";
