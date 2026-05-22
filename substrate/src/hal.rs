//! Hardware Abstraction Layer (HAL) for Project JARVIS.
//! Establish a hardware-agnostic interface for ARM64/x86_64/RISC-V.
//! Time: O(1) for hardware registration and interrupt dispatch.
//! Space: O(1) overhead.

/// Represents a physical hardware architecture.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Architecture {
    X86_64,
    Arm64,
    RiscV,
}

/// The Hardware Interface Trait.
/// Must be implemented by the low-level bootloader/kernel for each target.
pub trait HardwareInterface {
    /// Identifies the underlying architecture.
    fn get_arch(&self) -> Architecture;

    /// Configures a DMA region for high-speed stream mapping.
    /// Time: O(1).
    fn configure_dma(&self, start_addr: *mut u8, size: usize) -> Result<(), HalError>;

    /// Performs a zero-copy memory map of a stream buffer.
    /// Time: O(1).
    fn map_stream_zero_copy(&self, src: *mut u8, dest: *mut u8, size: usize) -> Result<(), HalError>;

    /// Registers a hardware interrupt handler for stream-graph synchronization.
    /// Time: O(1).
    fn register_interrupt(&self, vector: u8, handler: extern "C" fn()) -> Result<(), HalError>;

    /// Triggers a hardware trap for formal contract violations.
    /// Time: O(1).
    fn trigger_trap(&self, code: u32) -> !;

    /// Writes a single byte to the serial port for diagnostics.
    /// Time: O(1).
    fn write_serial(&self, byte: u8);
}

#[derive(Debug)]
pub enum HalError {
    InvalidAddress,
    DmaConfigurationFailed,
    InterruptRegistrationFailed,
    UnsupportedArchitecture,
}

/// Global HAL instance pointer, initialized during boot.
static mut HAL_INSTANCE: Option<&'static dyn HardwareInterface> = None;

/// Set the global HAL instance.
/// Time: O(1).
pub fn set_hal(instance: &'static dyn HardwareInterface) {
    unsafe {
        HAL_INSTANCE = Some(instance);
    }
}

/// Retrieve the current HAL instance.
/// Time: O(1).
pub fn get_hal() -> &'static dyn HardwareInterface {
    unsafe {
        HAL_INSTANCE.expect("HAL not initialized. Substrate failure.")
    }
}
