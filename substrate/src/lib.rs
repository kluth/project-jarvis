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

use crate::hal::{HardwareInterface, Architecture, HalError};
use crate::scheduler::Scheduler;

pub struct X86BootInterface;

impl HardwareInterface for X86BootInterface {
    fn get_arch(&self) -> Architecture { Architecture::X86_64 }
    fn configure_dma(&self, _addr: *mut u8, _size: usize) -> Result<(), HalError> { Ok(()) }
    fn map_stream_zero_copy(&self, _src: *mut u8, _dest: *mut u8, _size: usize) -> Result<(), HalError> { Ok(()) }
    fn register_interrupt(&self, _v: u8, _h: extern "C" fn()) -> Result<(), HalError> { Ok(()) }
    fn trigger_trap(&self, _code: u32) -> ! { loop {} }
    fn write_serial(&self, byte: u8) {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            core::arch::asm!("out dx, al", in("dx") 0x3f8u16, in("al") byte);
        }
        #[cfg(not(target_arch = "x86_64"))]
        let _ = byte;
    }
}

static BOOT_HAL: X86BootInterface = X86BootInterface;

/// The Native Entry Point for Multiboot2 Loaders.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    JarvisSubstrate::boot()
}

impl JarvisSubstrate {
    /// JARVIS Substrate Entry Point.
    /// Time: O(1) for hardware initialization.
    pub fn boot() -> ! {
        // 1. Initialize Hardware Interface
        crate::hal::set_hal(&BOOT_HAL);
        
        let hal = crate::hal::get_hal();
        for &c in b"JARVIS BOOT\n" {
            hal.write_serial(c);
        }

        // 2. Initialize Scheduler with null root node (bootstrap state)
        let scheduler = Scheduler::new(core::ptr::null_mut());

        // 3. Main Substrate Execution Loop
        loop {
            // PDD: O(1) scheduling tick
            scheduler.tick();
            
            // EFDD: Enter low-power wait state if no tasks pending
            unsafe {
                core::arch::asm!("hlt");
            }
        }
    }
}
