#![no_std]

extern crate alloc;

mod axdriver_net;
mod descriptor;
mod err;
mod igb;
mod phy;
mod regs;
mod ring;

use core::time::Duration;
pub use igb::*;

/// Vendor ID for Intel.
pub const INTEL_VEND: u16 = 0x8086;

/// Device ID for the 82576EB, used to identify the device from the PCI space.
pub const INTEL_82576: u16 = 0x10C9;

/// Device name
pub const DEVICE_NAME: &str = "igb";

pub trait Kernel {
    fn sleep(duration: Duration);
}

pub(crate) fn sleep(duration: Duration) {
    extern "Rust" {
        fn _igb_driver_sleep(duration: Duration);
    }

    unsafe {
        _igb_driver_sleep(duration);
    }
}

#[macro_export]
macro_rules! set_impl {
    ($t: ty) => {
        #[no_mangle]
        unsafe fn _igb_driver_sleep(duration: core::time::Duration) {
            <$t as $crate::Kernel>::sleep(duration)
        }
    };
}
