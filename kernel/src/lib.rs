#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub mod scheduler;
pub mod stream;
pub mod allocator;
pub mod evolution;

pub struct JarvisKernel;

impl JarvisKernel {
    pub fn boot() -> ! {
        // Initialize hardware, then start the stream-graph executor
        loop {
            // Poll stream nodes
        }
    }
}
