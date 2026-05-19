//! EFDD-Compliant Arena Allocator
//! Time: O(1) for allocation and reclamation.
//! Space: O(K) where K is alignment padding.

use core::alloc::Layout;
use core::sync::atomic::{AtomicPtr, Ordering};
use core::ptr;

/// A Cache-Line Aligned Arena Allocator designed for high-efficiency, 
/// zero-downtime node swapping.
/// 
/// EFDD: Minimizes energy waste by avoiding fragmented allocations and
/// ensuring cache-line alignment to reduce bus-width power consumption.
pub struct EfddArenaAllocator {
    start: *mut u8,
    end: *mut u8,
    current: AtomicPtr<u8>,
}

impl EfddArenaAllocator {
    /// Initializer for the arena.
    /// Time: O(1), Space: O(1)
    pub const fn new(buffer: *mut u8, size: usize) -> Self {
        Self {
            start: buffer,
            end: unsafe { buffer.add(size) },
            current: AtomicPtr::new(buffer),
        }
    }

    /// Allocates memory from the arena with 64-byte alignment.
    /// Time: O(1), Space: O(K) (Alignment overhead)
    pub fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = 64; // Force 64-byte alignment for EFDD compliance
        let size = layout.size();

        let mut current = self.current.load(Ordering::Acquire);
        loop {
            let addr = current as usize;
            let aligned_addr = (addr + align - 1) & !(align - 1);
            let next_addr = aligned_addr + size;
            let next = next_addr as *mut u8;

            if next > self.end {
                return ptr::null_mut();
            }

            match self.current.compare_exchange_weak(
                current, 
                next, 
                Ordering::AcqRel, 
                Ordering::Acquire
            ) {
                Ok(_) => return aligned_addr as *mut u8,
                Err(updated) => current = updated,
            }
        }
    }

    /// Resets the arena for a full memory reclamation.
    /// MUST only be called after verifying zero active references.
    /// Time: O(1), Space: O(1)
    pub fn reset(&self) {
        self.current.store(self.start, Ordering::Release);
    }
}
unsafe impl Sync for EfddArenaAllocator {}
