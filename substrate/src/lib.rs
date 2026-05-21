#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub mod scheduler;
pub mod stream;
pub mod allocator;
pub mod evolution;
pub mod substrate;
pub mod hal;

pub struct JarvisSubstrate;

impl JarvisSubstrate {
    pub fn boot() -> ! {
        // Initialize hardware, then start the stream-graph executor
        loop {
            // Poll stream nodes
        }
    }
}
